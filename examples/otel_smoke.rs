use anyhow::Result;
use build123d_cad::telemetry;
use rig::telemetry::SpanCombinator;

const OTEL_SERVICE_NAME: &str = "build123d-cad";

#[tokio::main]
async fn main() -> Result<()> {
    let telemetry = telemetry::init_telemetry(OTEL_SERVICE_NAME)?;
    let marker = std::env::var("OTEL_SMOKE_MARKER").unwrap_or_else(|_| "manual-run".to_string());

    let span = tracing::info_span!(
        "otel_smoke_probe",
        marker = %marker,
        run_entrypoint = "otel_smoke"
    );
    span.record_model_input(&serde_json::json!({
        "marker": marker,
        "purpose": "collector smoke check",
    }));
    let _guard = span.enter();

    tracing::info!("emitting OpenTelemetry smoke span");
    span.record_model_output(&serde_json::json!({
        "status": "ok",
    }));

    telemetry.shutdown()?;
    Ok(())
}
