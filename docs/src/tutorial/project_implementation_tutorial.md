# Tutorial: How to Implement and Learn From a New Feature in this Project

This document is a practical implementation playbook for this repository’s Rust+agent architecture.
It covers concept, syntax, implementation flow, troubleshooting, and the lessons learned while building and iterating this codebase, with a dedicated OpenTelemetry implementation focus.

## 1) What “implementing” means in this project

In this repo, implementation has five layers:

1. **Concept layer**: define what behavior you want (objective, safety scope, success criteria).
2. **Agent layer**: decide which role should reason, generate, or review.
3. **Tool layer**: expose the capability required by the agent.
4. **Orchestrator layer**: route tasks and decide when a handoff or retry is needed.
5. **Execution layer**: run generated artifacts (Python script, lints, tests, viewer output) and report outcomes.

Treat this as an API: inputs enter at one boundary (`src/main.rs` / CLI), move through orchestration (`src/agentic`), then return an auditable artifact (`.output/...`), plus traces.

## 2) Minimal concept map for a new capability

Use this map for every addition:

- **Input path**: where does the request begin?
  - `src/main.rs` for CLI objective and runtime flags.
- **Decision path**: which agent decides next?
  - `src/agentic/orchestrator.rs`
- **Capability path**: which specialist owns the work?
  - `src/agentic/agents/*`
- **Action path**: which tool executes side effects?
  - `src/agentic/tools/*`
- **Storage path**: where is output persisted and discoverable?
  - `src/agentic/tools/code_editor.rs`, `.output` workspace, viewer entrypoint.

If the answer cannot be mapped clearly across these five places, the design is usually incomplete.

## 3) Files you will usually edit

Use this sequence, not a fixed law:

- `src/agentic/prompts.rs` (if you need new role-specific prompt structure)
- `src/agentic/tools/*.rs` (for new external actions or validators)
- `src/agentic/agents/*.rs` (for role behavior)
- `src/agentic/orchestrator.rs` (for routing and stopping conditions)
- `src/agentic/mod.rs` and/or `src/main.rs` (for registration and wiring)
- `src/agentic/errors.rs` (if new failure semantics are introduced)

## 4) Concrete implementation sequence

### 4.1 Define the requirement in one paragraph

Write a one-line requirement first:

```text
Create <x> capability so that objective <A> produces <artifact B> and is gated by <review rule R>.
```

This forces scope discipline before touching code.

### 4.2 Add a new tool (capability)

1. Create a tool struct/function in `src/agentic/tools/`.
2. Keep tool input/output narrow and serializable.
3. Return explicit structured errors (`Result<T, ToolError>` style), not silent fallback.
4. Add traces around tool calls when the operation affects control flow.

Example skeleton:

```rust,ignore
pub async fn run_x_tool(input: ToolInput, ctx: &Context) -> Result<ToolOutput, ToolError> {
    // validate
    // perform action
    // return normalized output
}
```

### 4.3 Add a specialist agent

Create a new agent under `src/agentic/agents/` and implement the role contract used in existing agents:

```rust,ignore
pub struct NewAgent { /* config */ }

impl NewAgent {
    pub fn new(model: impl Clone + Send + Sync + 'static) -> Self { Self { model } }
}

// In run_turn or run method:
// - read global context
// - decide whether to keep working, delegate, or finalize
```

### 4.4 Register in orchestrator

Update routing so the orchestrator can route missions to the new role:

```rust,ignore
let mut agents = HashMap::new();
agents.insert("new_agent".to_string(), Box::new(NewAgent::new(model.clone())) as Box<dyn Specialist>);
```

Then add conditions in route selection logic for:

- when to invoke it
- when to stop and return results
- when to delegate back to reviewer paths

### 4.5 Add verification and guardrails

Every generation capability should have a validator:

- syntactic validation (Python/JSON/command syntax)
- policy checks (compliance/intended restrictions)
- runtime sanity checks (small deterministic example)

Use existing linter-like and verification helpers before calling an artifact final.

### 4.6 Add documentation and examples

For every code path change:

- update this docs area with a usage snippet
- include a minimal command example in bash
- include expected success criteria in text

## 5) Syntax playbook (what to copy, what to avoid)

### Copy (good)

- Keep tool signatures explicit.
- Use small context structs for prompt and results.
- Keep model calls and tool calls separated.
- Use telemetry fields consistently.

### Avoid (bad)

- Catch-all agents that do multiple specialized jobs.
- Implicit branching logic spread across unrelated files.
- Swallowing tool errors with `unwrap`/`expect`.
- Returning markdown blobs as machine state.

## 6) OpenTelemetry-first implementation in this project (special focus)

This repository treats observability as a design input, not an optional add-on.

### 6.1 Telemetry boot sequence

The entrypoint initializes telemetry before agents are created:

- file: `src/main.rs`

1. call `telemetry::init_telemetry(OTEL_SERVICE_NAME)`
2. keep `TelemetryHandle` alive for process lifetime
3. emit mission context through `rig_build123d_request` span
4. flush on shutdown (`telemetry.shutdown()`)

Why this matters:

- without global initialization, spans from agents/tools are not exported
- without proper shutdown, exporters may drop late spans under load

### 6.2 Stable root span pattern

Use one stable root span per objective so downstream traces are easy to query:

```rust,ignore
let mission_span = tracing::info_span!(
    "rig_build123d_request",
    request_id = %request_id,
    gen_ai_provider_name = "anthropic",
    gen_ai_system = "anthropic.azure",
    gen_ai_operation_name = "chat.completion",
    gen_ai_request_model = %model,
    gen_ai_error_type = "none",
    run_entrypoint = "cli",
    objective_len = objective.len()
);
mission_span.record_model_input(&serde_json::json!({
    "objective_preview": objective_preview,
    "objective_len": objective.len(),
    "workflow": "agentic_cad",
}));
```

This is how failures can be traced as a single mission and still drill into:

- `agent_orchestrator`
- `agent_turn`
- tool spans (`tool.*`)

### 6.3 Export pipeline implementation details

The project’s telemetry bootstrap is in `src/telemetry.rs` and implements:

- optional OTLP enable (`OTEL_ENABLED`)
- protocol resolution (`grpc` / `http/protobuf` / `http/json`)
- endpoint precedence:
  1. `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`
  2. `OTEL_EXPORTER_OTLP_ENDPOINT`
  3. `SIGNOZ_ENDPOINT`
- header precedence:
  1. `OTEL_EXPORTER_OTLP_TRACES_HEADERS`
  2. `OTEL_EXPORTER_OTLP_HEADERS`
  3. `SIGNOZ_INGESTION_KEY`
- compression and sampler handling
- service-level attributes (`service.name`, `service.version`, environment, pid)

### 6.4 OTel-focused implementation checklist for every new path

When adding a new tool/agent feature, enforce these rules:

1. Add or reuse a stable span name (low-cardinality, role-based, e.g. `agent.security_reviewer`).
2. Add `status`, `step`, and `agent` as structured fields (not in span name).
3. Emit `record_model_input` before risky operations and `record_model_output` at each terminal branch.
4. Preserve redaction policy (never place raw prompts or secrets in exported fields).
5. Ensure timeout and fallback spans exist for degraded mode.
6. Confirm `TelemetryHandle` still owns provider until process end.

### 6.5 OpenTelemetry setup examples

- local:

```bash
export OTEL_ENABLED=true
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"
export OTEL_EXPORTER_OTLP_PROTOCOL="grpc"
export OTEL_SERVICE_NAME="build123d-cad-cli"
export OTEL_TRACES_SAMPLER_ARG="1.0"
```

- SigNoz-compatible (cloud):

```bash
export OTEL_ENABLED=true
export OTEL_EXPORTER_OTLP_TRACES_ENDPOINT="https://<region>.ingest.signoz.cloud:443"
export OTEL_SERVICE_NAME="build123d-cad"
export OTEL_EXPORTER_OTLP_TRACES_HEADERS="signoz-ingestion-key=<key>"
export OTEL_EXPORTER_OTLP_TRACES_COMPRESSION="gzip"
```

Fallback mode (`OTEL_ENABLED=false`) is intentional for air-gapped or local debug runs.

For the complete OTEL variable matrix and collector setup examples, see [`docs/src/observability.md`](../observability.md).

### 6.6 OTel troubleshooting playbook

#### Symptom: no spans in backend

1. confirm `init_telemetry` ran before agent initialization.
2. verify `OTEL_ENABLED=true` and endpoint values.
3. check protocol and header format (`OTEL_EXPORTER_OTLP_*` vs `OTEL_EXPORTER_OTLP_TRACES_*`).
4. run `cargo run --example otel_smoke` and validate probe span.

#### Symptom: spans present but context is fragmented

1. verify mission-level scope is a single active root span.
2. ensure each delegation path enters child spans consistently.
3. avoid creating new detached spans for equivalent steps.

#### Symptom: noisy span counts / high cardinality

1. stop putting long/free text in span attributes.
2. keep span names fixed and put values into attributes.
3. reduce verbose debug logs while keeping `record_model_output` at branch boundaries only.

## 7) Troubleshooting playbook from real implementation experience

### Symptom: mission loops forever

**Likely causes**

- weak stop condition in orchestrator
- reviewer and coder bouncing without diff changes
- missing convergence criteria in policy

**Fix**

1. Inspect `agent_turn` span frequency and transition labels.
2. Check if the same `code_snippet` is proposed repeatedly.
3. Add an explicit step cap and fallback route to `Broken`/manual-review state.

### Symptom: generated Python fails but review says “ok”

**Likely causes**

- review prompt too narrow
- tool output validation done after review, not before finalization

**Fix**

1. move syntax/lint check earlier in the loop.
2. enrich review prompt with concrete failure patterns.
3. fail closed: `review pass` only after deterministic checks pass.

### Symptom: weak quality on specific objectives

**Likely causes**

- too much context in one turn
- missing retrieval grounding
- incorrect tool/toolchain selection

**Fix**

1. split objective into composable constraints.
2. route to specialist agent earlier.
3. prefer deterministic retrieval first, then generative synthesis.

### Symptom: noisy traces with low signal

**Likely causes**

- over-verbose event names
- missing stable span names
- no stable attribute schema

**Fix**

1. keep span naming fixed (`agent_orchestrator`, `agent_turn`, `tool.xxx`).
2. emit compact structured attributes (role, step, status).
3. aggregate by step and model name before reacting.

## 8) Learning insights from implementing this repo

### What worked

- **Strong separation of roles** beats one large “super agent.”
- **Self-healing** (review + re-run + refine) is more reliable than one-shot generation.
- **Observability-first onboarding** made regressions diagnosable in minutes.
- **Knowledge persistence** reduced repeated mistakes when similar objectives reappeared.

### What was expensive/fragile

- broad agent prompts with weak output schema
- late validation (too much after the fact)
- inconsistent resource boundaries (web lookup + generation + execution order ambiguity)

### What we keep as policy

- every generation path ends with verification
- every production path has a fallback `Broken`/manual-review representation
- every new capability adds one traceable decision point in orchestrator

## 9) Best practices for future implementation

1. Start with telemetry before scaling prompt complexity.
2. Add smallest meaningful failure unit first.
3. Keep role contracts narrow and explicit.
4. Add a local example and a deterministic test command.
5. Treat archive links and changelog notes as part of implementation.
6. Prefer readable control flow over clever abstractions.
7. Write docs in the same PR as code so onboarding cost stays low.

## 10) Suggested command flow

```bash
# run mission once
cargo run -- "Create a parametric flange with 6 bolt holes"

# inspect generated output
ls .output

# run checks when implementing new behavior
cargo test
cargo clippy --all-targets --all-features
```

## 11) Final checklist before merge

- [ ] Concept is expressed in one paragraph.
- [ ] Responsibility boundaries are explicit.
- [ ] Tool contracts are typed and return deterministic errors.
- [ ] Orchestrator route includes stop and fallback rules.
- [ ] Validation/trace points added.
- [ ] Docs and example updated.
- [ ] Telemetry impact reviewed for the new path.
