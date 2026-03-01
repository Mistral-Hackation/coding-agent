# Introduction

Welcome to the **build123d CAD Code Generator** documentation.

This project is an Anthropic/Azure-powered multi-agent orchestrator that generates build123d Python scripts, validates them through review gates, executes them with `uv`, and optionally opens a local 3D viewer.

## Core goals

- turn natural-language objectives into executable parametric CAD scripts
- enforce review quality gates before finalization
- preserve run artifacts and checkpoints for debugging/reproducibility
- provide end-to-end traces for request, orchestration, and tool phases

## Runtime model

The active system is the `agentic` architecture:

1. Supervisor routes work to specialists.
2. Researcher gathers context.
3. Coder generates code.
4. Reviewer pipeline validates quality, geometry, intent, and compliance.
5. Supervisor finalizes or delegates revisions.
6. Orchestrator writes output, executes code, and captures artifacts.

## Provider model

This repository currently uses:

- **Anthropic models** via **Azure MaaS**
- configurable model through `AZURE_MODEL` (default `claude-opus-4-5`)

No Gemini provider is required for normal operation.

If you are adapting external tutorials that mention Gemini, keep the same tracing shape and only
replace the provider client/model wiring for Azure Anthropic.

## Observability model

OpenTelemetry instrumentation is built in:

- root request spans (`rig_build123d_request`)
- orchestrator spans (`agent_orchestrator`, `agent_turn`)
- tool spans (`tool.*`)

See [Observability](observability.md) for setup and runbook details.
