# Tutorial: Add a Custom Agent

This tutorial shows how to add a new specialist without changing provider setup (Anthropic/Azure remains unchanged).

## 1) Implement the `Specialist` contract

Create a new file under `src/agentic/agents/`, for example `security_reviewer.rs`.

Implement:

- `fn name(&self) -> &str`
- `async fn run_turn(&self, ctx: GlobalContext) -> Result<TurnResult, AgentError>`

## 2) Build an agent persona

Use the same completion model type used by existing agents:

```rust,ignore
pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self
```

Attach only the tools needed for that role.

## 3) Add deterministic routing behavior

Inside `run_turn`, return one of:

- `TurnResult::KeepWorking`
- `TurnResult::Delegate`
- `TurnResult::FinalResult`

Use explicit delegation targets that already exist in the orchestrator agent map.

## 4) Register the agent

Update `src/main.rs` (and example binaries if needed):

1. construct the new agent with `model.clone()`
2. insert it into the `HashMap<String, Box<dyn Specialist>>`
3. ensure Supervisor prompt/router logic is aware of the new agent key

## 5) Instrument your new agent

Prefer stable phase spans to keep traces queryable:

- `agent.security_reviewer` (or similar stable name)
- include concise attributes (role, step, status)
- avoid using full prompt text as span names

For custom spans, use `SpanCombinator` to record model I/O safely:

```rust,ignore
use rig::telemetry::SpanCombinator;

let span = tracing::info_span!("agent.security_reviewer", role = "security");
span.record_model_input(&serde_json::json!({ "task": "review generated code" }));
let _guard = span.enter();
// ... model/tool calls ...
span.record_model_output(&serde_json::json!({ "status": "approved" }));
```

## 6) Validate

Run:

```bash
cargo test
cargo clippy --all-targets --all-features
```

If telemetry is enabled, also verify trace shape in your OTLP backend:

- request root under `rig_build123d_request`
- per-step orchestration under `agent_turn`
- your custom agent span nested in the correct step context
