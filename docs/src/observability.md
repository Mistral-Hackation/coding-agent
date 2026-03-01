# Observability

This project supports production-oriented tracing using **OpenTelemetry** and **SigNoz-compatible OTLP collectors**.

Current provider wiring in this repository is **Anthropic models via Azure MaaS**.
No Gemini-specific provider setup is used by default.

## What is instrumented

### Request root

- span name: `rig_build123d_request`
- emitted from CLI and example entrypoints
- includes request id, provider/model metadata, objective length/preview

### Orchestration phases

- span name: `agent_orchestrator`
- span name: `agent_turn`
- includes step number, active agent, history/artifact counts, transition outcome

### Tool calls

- span names: `tool.web_search`, `tool.knowledge_base`, `tool.learn_knowledge`,
  `tool.example_search`, `tool.python_linter`, `tool.search_and_replace`,
  `tool.code_snippet_search`, `tool.code_snippet_replace`, `tool.git_snapshot`

## Environment variables

### Core telemetry

- `OTEL_ENABLED` (`true`/`false`, default `true`)
- `OTEL_SERVICE_NAME` (optional; defaults to entrypoint-specific service name, used as `service.name`)
- `OTEL_EXPORTER_OTLP_ENDPOINT` (default `http://localhost:4317`)
- `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` (optional override, precedence)
- `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL` (`grpc`, `http/protobuf`, `http/json`; default `grpc`)
- `OTEL_EXPORTER_OTLP_HEADERS` (for cloud ingestion keys)
- `OTEL_EXPORTER_OTLP_TRACES_HEADERS` (preferred trace header key; fallback to OTEL_EXPORTER_OTLP_HEADERS)
- `SIGNOZ_ENDPOINT` (fallback when `OTEL_EXPORTER_OTLP_ENDPOINT` is not set)
- `SIGNOZ_INGESTION_KEY` (fallback when `OTEL_EXPORTER_OTLP_HEADERS` is not set)
- `OTEL_EXPORTER_OTLP_TRACES_COMPRESSION` (`gzip` for enabled compression; omit/empty for no compression)
- `OTEL_EXPORTER_OTLP_COMPRESSION` (fallback if trace-specific compression is not set)
- `OTEL_TRACES_SAMPLER_ARG` (`0.0..=1.0`, default `1.0`)
- `DEPLOYMENT_ENVIRONMENT` (default `development`)
- `RUST_LOG` (default `info`)

### Application provider

- `AZURE_API_KEY`
- `AZURE_EXISTING_AIPROJECT_ENDPOINT`
- `AZURE_MODEL` (optional)
- `SERPER_API_KEY` (optional, required for WebSearcher paths)

## Setup examples

### SigNoz Cloud

```bash
export OTEL_ENABLED=true
export OTEL_EXPORTER_OTLP_ENDPOINT="https://<region>.ingest.signoz.cloud:443"
export OTEL_SERVICE_NAME="build123d-cad"
export OTEL_EXPORTER_OTLP_PROTOCOL="grpc" # or http/protobuf, http/json
export OTEL_EXPORTER_OTLP_HEADERS="signoz-ingestion-key=<ingestion-key>"
export OTEL_EXPORTER_OTLP_TRACES_COMPRESSION="gzip"
export OTEL_TRACES_SAMPLER_ARG="1.0"
```

If you already export SigNoz compatibility variables, this also works:

```bash
export OTEL_ENABLED=true
export SIGNOZ_ENDPOINT="https://<region>.ingest.signoz.cloud:443"
export OTEL_SERVICE_NAME="build123d-cad"
export SIGNOZ_INGESTION_KEY="<ingestion-key>"
export OTEL_EXPORTER_OTLP_TRACES_COMPRESSION="gzip"
export OTEL_TRACES_SAMPLER_ARG="1.0"
```

### Local collector

```bash
export OTEL_ENABLED=true
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
export OTEL_SERVICE_NAME="build123d-cad"
export OTEL_EXPORTER_OTLP_PROTOCOL="grpc"
unset OTEL_EXPORTER_OTLP_HEADERS
export OTEL_TRACES_SAMPLER_ARG="1.0"
```

Tip: keep one service name across entrypoints while debugging trace shape:

- `cargo run -- "..."` (CLI)
- `cargo run --example agentic_workflow`
- `cargo run --example otel_smoke`

To limit run time during tuning, set:

```bash
export AGENTIC_MAX_STEPS=20
export WEB_SEARCH_TIMEOUT_SECS=15
```

## Smoke test

Run:

```bash
OTEL_SMOKE_MARKER="manual-check" cargo run --example otel_smoke
```

Expected probe span:

- `otel_smoke_probe`

If your collector is connected correctly, the marker should appear in collector output and in your trace backend.

## Production notes

1. Keep span names stable and low-cardinality.
2. Put request details in attributes/events, not span names.
3. Keep prompt content truncated/redacted in telemetry fields.
4. Use generated traces with GenAI semantic conventions:
   - `gen_ai_provider_name="anthropic"`
   - `gen_ai_system="anthropic.azure"`
   - `gen_ai_operation_name` (for example `chat.completion`, `agent.workflow`)
   - `gen_ai_request_model` (for example `claude-opus-4-5`)
5. Initialize telemetry once at startup and always call shutdown.
6. Keep `AGENTIC_MAX_STEPS` and `WEB_SEARCH_TIMEOUT_SECS` low for fast iterative runs.
7. Validate trace shape first before tuning sampling/compression.
