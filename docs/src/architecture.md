# Architecture Deep Dive

This document summarizes the current runtime architecture.

## Request flow

```text
Objective
  -> Supervisor
  -> Researcher
  -> Coder
  -> Reviewer
  -> PhysicsReviewer
  -> IntentReviewer
  -> ComplianceReviewer
  -> Supervisor (finalize or revise)
  -> script write + auto-execution + optional viewer
```

## Consensus model

- `ReviewConsensus` tracks 4 reviewer approvals.
- Full consensus (`4/4`) is required before finalization.
- Reviewer failures can route back to `coder` or `researcher`.

## Main modules

- `src/main.rs`: CLI, provider setup, telemetry bootstrap, mission loop
- `src/telemetry.rs`: OpenTelemetry + tracing subscriber setup and shutdown
- `src/agentic/orchestrator.rs`: state machine loop, step transitions, output handling
- `src/agentic/agents/*`: specialist agent logic
- `src/agentic/tools/*`: tool integrations used by specialists
- `src/viewer.rs`: local Three.js STL viewer generation

## Observability architecture

### Span hierarchy

- request root: `rig_build123d_request`
- orchestration: `agent_orchestrator`
- per-step: `agent_turn`
- tools: `tool.*`

### Why this shape

- request-level filtering: one root span per objective
- phase-level latency: identify slow planner/reviewer/tool segments
- failure localization: transition and tool status are visible in trace context

## Output and reproducibility

Each mission writes to `.output/<project>_<timestamp>/` and snapshots key checkpoints:

- `generated_script_*.py`
- periodic context/code snapshots
- CAD output files (`.step`, `.stl`, `.svg`)
- optional viewer HTML (`viewer.html`)
- git snapshots for timeline recovery

For full details, see root [ARCHITECTURE.md](../../ARCHITECTURE.md).
