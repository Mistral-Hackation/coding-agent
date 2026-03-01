//! OpenTelemetry bootstrap and lifecycle management.
//!
//! This module initializes a `tracing` subscriber with optional OTLP export
//! to SigNoz (or any OpenTelemetry-compatible collector).
//!
//! ## Environment variables
//!
//! - `OTEL_ENABLED` (`true`/`false`, default: `true`)
//! - `OTEL_SERVICE_NAME` (optional, default: entrypoint-specific service name)
//! - `OTEL_EXPORTER_OTLP_ENDPOINT` (optional, default by protocol: localhost 4317/4318)
//! - `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` (optional override, precedence over `OTEL_EXPORTER_OTLP_ENDPOINT`)
//! - `SIGNOZ_ENDPOINT` (fallback when OTLP endpoint vars are not set)
//! - `OTEL_EXPORTER_OTLP_TRACES_HEADERS` (for example `signoz-ingestion-key=...`)
//! - `OTEL_EXPORTER_OTLP_HEADERS` (fallback when `OTEL_EXPORTER_OTLP_TRACES_HEADERS` is not set)
//! - `SIGNOZ_INGESTION_KEY` (fallback when OTLP header vars are not set)
//! - `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL` (preferred; optional; `grpc`, `http/protobuf`, `http/json`)
//! - `OTEL_EXPORTER_OTLP_PROTOCOL` (`grpc`, `http/protobuf`, `http/json`; default `grpc`)
//! - `OTEL_EXPORTER_OTLP_INSECURE` (`true`/`false`, default: `false` for HTTPS endpoints)
//! - `.env` in the current working directory or any parent directory (loaded automatically)
//! - `OTEL_EXPORTER_OTLP_COMPRESSION` (`gzip` for compression; omit/empty for no compression)
//! - `OTEL_EXPORTER_OTLP_TRACES_COMPRESSION` (optional, same format as `OTEL_EXPORTER_OTLP_COMPRESSION`; precedence)
//! - `OTEL_TRACES_SAMPLER_ARG` (`0.0..=1.0`, default: `1.0`)
//! - `RUST_LOG` (default: `info`)

use anyhow::{Context, Result, anyhow};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{
    Compression, Protocol, SpanExporter, WithExportConfig, WithHttpConfig, WithTonicConfig,
};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{Sampler, SdkTracerProvider};
use std::collections::HashMap;
use std::path::PathBuf;
use tonic::metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue};
use tonic::transport::ClientTlsConfig;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Handle returned by [`init_telemetry`].
///
/// Keep this handle alive for the full process lifetime. Dropping (or explicitly
/// calling [`TelemetryHandle::shutdown`]) flushes and closes the tracer provider.
pub struct TelemetryHandle {
    tracer_provider: Option<SdkTracerProvider>,
    otel_enabled: bool,
}

impl TelemetryHandle {
    /// Returns whether OTLP export is enabled.
    pub fn is_enabled(&self) -> bool {
        self.otel_enabled
    }

    /// Flushes pending spans and shuts down the tracer provider.
    pub fn shutdown(mut self) -> Result<()> {
        if let Some(provider) = self.tracer_provider.take() {
            provider
                .shutdown()
                .context("failed to shut down OpenTelemetry tracer provider")?;
        }
        Ok(())
    }

    fn disabled() -> Self {
        Self {
            tracer_provider: None,
            otel_enabled: false,
        }
    }
}

impl Drop for TelemetryHandle {
    fn drop(&mut self) {
        if let Some(provider) = self.tracer_provider.take() {
            let _ = provider.shutdown();
        }
    }
}

/// Initializes process-wide tracing and optional OTLP export.
///
/// This function must be called exactly once per process before any mission is run.
///
/// # Errors
///
/// Returns an error if:
///
/// - tracing subscriber initialization fails, or
/// - OTLP exporter creation fails when telemetry is enabled.
pub fn init_telemetry(service_name: &str) -> Result<TelemetryHandle> {
    let dotenv_path = ensure_dotenv_loaded();

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let otel_enabled = read_bool_env("OTEL_ENABLED", true);
    if !otel_enabled {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(tracing_subscriber::fmt::layer().with_target(false))
            .try_init()
            .context("failed to initialize tracing subscriber")?;
        tracing::info!("OpenTelemetry export disabled via OTEL_ENABLED=false");
        return Ok(TelemetryHandle::disabled());
    }

    let protocol = resolve_otlp_protocol();
    let (endpoint, endpoint_source) = resolved_otlp_endpoint(protocol);
    let otel_insecure = read_bool_env("OTEL_EXPORTER_OTLP_INSECURE", false);
    let endpoint_uri = endpoint.clone();
    let service_name = resolve_service_name(service_name);

    let (configured_headers, headers_source) = resolve_otlp_headers();
    let headers = parse_otlp_headers(configured_headers.clone())?;
    let http_headers = parse_otlp_headers_for_http(configured_headers)?;
    let compression = parse_otlp_compression()?;

    let exporter = match protocol {
        Protocol::HttpBinary | Protocol::HttpJson => {
            tracing::info!(
                endpoint = %endpoint_uri,
                protocol = ?protocol,
                "OpenTelemetry protocol set to HTTP transport"
            );
            let mut exporter_builder = SpanExporter::builder()
                .with_http()
                .with_protocol(protocol)
                .with_endpoint(endpoint)
                .with_http_client(reqwest::Client::new());

            if let Some(headers) = http_headers {
                exporter_builder = exporter_builder.with_headers(headers);
            }

            exporter_builder
                .build()
                .context("failed to create OTLP HTTP span exporter")?
        }
        Protocol::Grpc => {
            if is_https_endpoint(&endpoint_uri) && !otel_insecure {
                tracing::info!(endpoint = %endpoint_uri, "OpenTelemetry HTTPS endpoint detected; TLS enabled");
            } else if !is_https_endpoint(&endpoint_uri) && !otel_insecure {
                tracing::debug!(endpoint = %endpoint_uri, "OpenTelemetry endpoint is non-HTTPS; using plaintext transport");
            } else if otel_insecure {
                tracing::warn!(
                    endpoint = %endpoint_uri,
                    "OpenTelemetry insecure mode forced by OTEL_EXPORTER_OTLP_INSECURE=true"
                );
            }

            let mut exporter_builder = SpanExporter::builder()
                .with_tonic()
                .with_protocol(protocol)
                .with_endpoint(endpoint);

            if matches!(protocol, Protocol::Grpc)
                && !otel_insecure
                && is_https_endpoint(&endpoint_uri)
            {
                exporter_builder = exporter_builder
                    .with_tls_config(tls_config_for_https(&endpoint_uri).with_enabled_roots());
            }

            if let Some(metadata) = headers {
                exporter_builder = exporter_builder.with_metadata(metadata);
            }
            if let Some(compression) = compression {
                exporter_builder = exporter_builder.with_compression(compression);
            }

            exporter_builder
                .build()
                .context("failed to create OTLP gRPC span exporter")?
        }
    };

    let sample_ratio = parse_sampler_ratio();
    let tracer_provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            sample_ratio,
        ))))
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_service_name(service_name.to_owned())
                .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")))
                .with_attribute(KeyValue::new(
                    "service.instance.id",
                    format!("pid-{}", std::process::id()),
                ))
                .with_attribute(KeyValue::new(
                    "deployment.environment",
                    std::env::var("DEPLOYMENT_ENVIRONMENT")
                        .unwrap_or_else(|_| "development".to_string()),
                ))
                .with_attribute(KeyValue::new("telemetry.sdk.language", "rust"))
                .build(),
        )
        .build();

    global::set_tracer_provider(tracer_provider.clone());
    let tracer = tracer_provider.tracer("build123d-cad-tracer");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(otel_layer)
        .try_init()
        .context("failed to initialize tracing subscriber with OpenTelemetry layer")?;

    tracing::info!(
        otlp_enabled = true,
        endpoint = %endpoint_uri,
        endpoint_source = endpoint_source,
        protocol = ?protocol,
        service_name = %service_name,
        headers_source = headers_source.unwrap_or("<none>"),
        sample_ratio,
        otel_insecure,
        dotenv_path = match &dotenv_path {
            Some(path) => path.display().to_string(),
            None => "<none>".to_string(),
        },
        "OpenTelemetry initialized"
    );

    Ok(TelemetryHandle {
        tracer_provider: Some(tracer_provider),
        otel_enabled: true,
    })
}

/// Loads `.env` from the current directory or parent directories if present.
///
/// This is intentionally non-fatal when no `.env` exists, allowing callers that
/// rely on environment-first configuration to continue.
pub fn ensure_dotenv_loaded() -> Option<PathBuf> {
    load_dotenv_file()
}

fn resolve_service_name(default_service_name: &str) -> String {
    std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default_service_name.to_string())
}

fn read_bool_env(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn parse_sampler_ratio() -> f64 {
    std::env::var("OTEL_TRACES_SAMPLER_ARG")
        .ok()
        .and_then(|raw| raw.parse::<f64>().ok())
        .map(|ratio| ratio.clamp(0.0, 1.0))
        .unwrap_or(1.0)
}

fn parse_otlp_compression() -> Result<Option<Compression>> {
    let value = std::env::var("OTEL_EXPORTER_OTLP_TRACES_COMPRESSION")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::env::var("OTEL_EXPORTER_OTLP_COMPRESSION")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .unwrap_or_default();

    match value.trim().to_ascii_lowercase().as_str() {
        "" => Ok(None),
        "gzip" => Ok(Some(Compression::Gzip)),
        "none" => Err(anyhow!(
            "unsupported OTEL_EXPORTER_OTLP_COMPRESSION value 'none'; use empty/omit to disable compression"
        )),
        unsupported => Err(anyhow!(
            "unsupported OTEL_EXPORTER_OTLP_COMPRESSION value '{}': supported values are gzip, or empty/unset",
            unsupported
        )),
    }
}

fn resolve_otlp_protocol() -> Protocol {
    let raw_protocol = std::env::var("OTEL_EXPORTER_OTLP_TRACES_PROTOCOL")
        .or_else(|_| std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL"))
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .to_string();

    match raw_protocol.as_str() {
        "" | "grpc" => Protocol::Grpc,
        "grpc/protobuf" => Protocol::Grpc,
        "http/protobuf" => Protocol::HttpBinary,
        "http/json" => Protocol::HttpJson,
        unsupported => {
            tracing::warn!(
                protocol = %unsupported,
                "Unsupported OTEL_EXPORTER_OTLP_TRACES_PROTOCOL/OTEL_EXPORTER_OTLP_PROTOCOL, using grpc"
            );
            Protocol::Grpc
        }
    }
}

fn is_https_endpoint(endpoint: &str) -> bool {
    endpoint.trim().to_ascii_lowercase().starts_with("https://")
}

fn tls_config_for_https(endpoint: &str) -> ClientTlsConfig {
    let host = endpoint
        .trim()
        .trim_start_matches("https://")
        .split('/')
        .next()
        .unwrap_or(endpoint)
        .split('?')
        .next()
        .unwrap_or(endpoint)
        .split(':')
        .next()
        .unwrap_or("ingest.eu.signoz.cloud")
        .to_string();

    ClientTlsConfig::new().domain_name(host)
}

fn resolved_otlp_endpoint(protocol: Protocol) -> (String, &'static str) {
    if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT") {
        return (endpoint, "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT");
    }

    if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        return (endpoint, "OTEL_EXPORTER_OTLP_ENDPOINT");
    }

    if let Ok(endpoint) = std::env::var("SIGNOZ_ENDPOINT") {
        return (endpoint, "SIGNOZ_ENDPOINT");
    }

    let default_endpoint = match protocol {
        Protocol::HttpBinary | Protocol::HttpJson => "http://localhost:4318",
        Protocol::Grpc => "http://localhost:4317",
    };

    (default_endpoint.to_string(), "default")
}

fn resolve_otlp_headers() -> (Option<String>, Option<&'static str>) {
    if let Ok(headers) = std::env::var("OTEL_EXPORTER_OTLP_TRACES_HEADERS") {
        return (Some(headers), Some("OTEL_EXPORTER_OTLP_TRACES_HEADERS"));
    }

    if let Ok(headers) = std::env::var("OTEL_EXPORTER_OTLP_HEADERS") {
        return (Some(headers), Some("OTEL_EXPORTER_OTLP_HEADERS"));
    }

    let signoz = resolve_signoz_headers();
    if signoz.is_some() {
        return (signoz, Some("SIGNOZ_INGESTION_KEY"));
    }

    (None, None)
}

fn resolve_signoz_headers() -> Option<String> {
    std::env::var("SIGNOZ_INGESTION_KEY").ok().and_then(|key| {
        let trimmed = key.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(format!("signoz-ingestion-key={trimmed}"))
        }
    })
}

fn parse_otlp_headers(headers: Option<String>) -> Result<Option<MetadataMap>> {
    let Some(headers) = headers else {
        return Ok(None);
    };

    let mut metadata = MetadataMap::new();
    for pair in headers.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (name, value) = pair.split_once('=').ok_or_else(|| {
            anyhow!(
                "invalid OTEL_EXPORTER_OTLP_HEADERS entry '{}'; expected key=value",
                pair
            )
        })?;

        let key: MetadataKey<Ascii> = name
            .trim()
            .parse()
            .with_context(|| format!("invalid OTLP header name '{}'", name.trim()))?;
        let metadata_value = MetadataValue::try_from(value.trim())
            .with_context(|| format!("invalid OTLP header value for key '{}'", name.trim()))?;
        metadata.insert(key, metadata_value);
    }

    Ok(Some(metadata))
}

fn parse_otlp_headers_for_http(headers: Option<String>) -> Result<Option<HashMap<String, String>>> {
    let Some(headers) = headers else {
        return Ok(None);
    };

    let mut parsed = HashMap::new();
    for pair in headers.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (name, value) = pair.split_once('=').ok_or_else(|| {
            anyhow!(
                "invalid OTEL_EXPORTER_OTLP_HEADERS entry '{}'; expected key=value",
                pair
            )
        })?;
        parsed.insert(name.trim().to_string(), value.trim().to_string());
    }

    Ok(Some(parsed))
}

fn load_dotenv_file() -> Option<PathBuf> {
    let dotenv_path = discover_dotenv_path()?;

    match dotenvy::from_path(dotenv_path.as_path()) {
        Ok(()) => Some(dotenv_path),
        Err(_) => None,
    }
}

fn discover_dotenv_path() -> Option<PathBuf> {
    let mut directory = std::env::current_dir().ok()?;

    for _ in 0..6 {
        let candidate = directory.join(".env");
        if candidate.is_file() {
            return Some(candidate);
        }

        if !directory.pop() {
            break;
        }
    }

    None
}
