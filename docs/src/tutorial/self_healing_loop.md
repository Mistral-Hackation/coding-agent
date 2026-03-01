# Tutorial: Self-Healing Loop

This tutorial explains the repair loop concept used in this repository and how it maps to current code.

## Goal

Expect generated code to fail sometimes, then recover through structured retries instead of terminating immediately.

## Two implementation layers in this repo

### Active path: multi-agent orchestrator

- file: `src/agentic/orchestrator.rs`
- behavior: reviewer feedback can route work back to researcher/coder
- convergence: finalize only after review consensus

## State types and resilience in runtime

The project uses state-indexed artifacts during orchestration (`Blueprint -> Draft -> Product`/`Broken`) to keep transitions explicit and enable recovery:

- explicit transition semantics
- explicit error carry-forward (`Broken` keeps code + diagnosis)
- testability of recovery logic

## Observability in self-healing paths

When analyzing retries, use these spans:

- `agent_orchestrator`: mission-level coordination
- `agent_turn`: per-step transitions
- `tool.code_snippet_replace`: deterministic patch actions
- `tool.python_linter`: syntax verification outcomes

This allows you to answer:

- where retries cluster
- whether failures are model planning, tool errors, or execution errors
- whether repairs converge or loop

## Practical debugging sequence

1. Run one mission objective.
2. Inspect trace tree for repeated `agent_turn` delegations.
3. Check latest `tool.*` spans before failure.
4. Validate generated script snapshots in `.output/...`.
5. Tune prompts/tools only after instrumentation is complete.
