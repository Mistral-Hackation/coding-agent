# Architecture Deep Dive: Multi-Agent Orchestrator with Quality Review Pipeline

This document explains the **Agentic Architecture** implemented in the build123d Automation project. It uses a dynamic, reasoning-driven state machine with **multi-stage quality review**.

---

## 1. Core Paradigm: The "Conch Shell" Protocol

We use a **Supervisor-Worker State Machine** where control is explicitly passed between agents. This is the "Conch Shell" pattern: only the agent holding the shell can act.

### The Flow (with Review Pipeline)
```
┌─────────────┐
│  Objective  │
└──────┬──────┘
       ▼
┌─────────────┐
│ Supervisor  │ ◄── Routes to appropriate specialist (Final Authority)
└──────┬──────┘
       ▼
┌─────────────┐
│ Researcher  │ ◄── Uses WebSearcher to gather info
└──────┬──────┘
       ▼
┌─────────────┐
│   Coder     │ ◄── Writes build123d Python script + validates via LinterTool
└──────┬──────┘
       ▼
┌─────────────┐
│  Reviewer   │ ◄── Code quality review (marks approval in ReviewConsensus)
└──────┬──────┘
       ▼
┌─────────────────┐
│ PhysicsReviewer │ ◄── Physics/geometry accuracy
└──────┬──────────┘
       ▼
┌─────────────────┐
│ IntentReviewer  │ ◄── User intent verification
└──────┬──────────┘
       ▼
┌────────────────────┐
│ ComplianceReviewer │ ◄── EU regulations (CE, ATEX, PED) — FINAL GATE
└──────┬─────────────┘
       ▼ (ALL_REVIEWS_COMPLETE — 4/4 consensus)
┌─────────────────┐
│ Supervisor      │ ◄── FINAL AUTHORITY: FINALIZE or DELEGATE
└──────┬──────────┘
       ▼
┌─────────────────┐
│ Auto-Execution  │ ◄── uv run --with build123d python3 script.py
└──────┬──────────┘
       ▼
┌─────────────────┐
│  3D Viewer      │ ◄── Three.js HTML viewer auto-opens in browser
└─────────────────┘
```

**Any reviewer can send REVISE signals back to Coder or Researcher**, enabling iterative refinement until all quality gates are passed.

---

## 2. Agent Roster (7 Agents)

| Agent | Type | Role | Tools |
|-------|------|------|-------|
| **Supervisor** | Router | Routes tasks, has **final authority** after consensus | — |
| **Researcher** | Worker | Gathers information from web | 📚 KB, 🧠 Learn, 🛠️ Web |
| **Coder** | Worker | Writes build123d Python scripts | 📚 KB, 🧠 Learn, 🛠️ Web, 🔍 Search, ✏️ Replace, 🐍 Linter |
| **Reviewer** | Quality Gate | Code quality & best practices | 📚 KB, 🧠 Learn, 🛠️ Web, 🔍 Search, 🐍 Linter |
| **PhysicsReviewer** | Quality Gate | Physics, geometry, 3D accuracy | 📚 KB, 🧠 Learn, 🛠️ Web, 🐍 Linter |
| **IntentReviewer** | Quality Gate | User intent verification | 📚 KB, 🧠 Learn, 🛠️ Web, 🐍 Linter |
| **ComplianceReviewer** | Quality Gate (Final) | EU regulations (CE, ATEX, PED) | 📚 KB, 🧠 Learn, 🛠️ Web, 🐍 Linter |

### Tool Legend

| Symbol | Tool | Purpose |
|--------|------|---------|
| 📚 KB | KnowledgeBase | Read agent-specific domain expertise |
| 🧠 Learn | LearnKnowledge | Save discoveries for future runs |
| 🛠️ Web | WebSearcher | Search the web via serper.dev |
| 🔍 Search | CodeSnippetSearch | Find patterns in generated code with context |
| ✏️ Replace | CodeSnippetReplace | Atomic search-and-replace on generated code |
| 🐍 Linter | LinterTool | Validate Python syntax (`py_compile`) |

---

## 3. Control Signals (`TurnResult`)

Each agent returns one of these signals:

```rust
pub enum TurnResult {
    /// Agent needs more iterations (thinking)
    KeepWorking { thought: String, new_context: GlobalContext },
    
    /// Route control to another agent
    Delegate { target_agent: String, instruction: String, new_context: GlobalContext },
    
    /// Mission complete - return final output
    FinalResult(String),
}
```

---

## 4. Layers & Modules

### A. The Agentic Core (`src/agentic/`)

| File | Purpose |
|------|---------|
| `mod.rs` | `GlobalContext`, `ReviewConsensus`, `TurnResult`, `Specialist` trait |
| `agents/` | All 7 agents (Supervisor, Researcher, Coder, + 4 Reviewers) |
| `orchestrator.rs` | Main loop, auto-execution (uv), snapshots, code sanitization |
| `errors.rs` | `AgentError` using `thiserror` |
| `tools/web_searcher.rs` | WebSearcher tool (serpscraper integration) |
| `tools/knowledge_base.rs` | KnowledgeBase (read) + LearnKnowledge (write) tools |
| `tools/code_editor.rs` | CodeSnippetSearch + CodeSnippetReplace tools |
| `tools/example_searcher.rs` | ExampleSearcher (build123d example corpus) |

### B. Viewer (`src/viewer.rs`)

Generates a self-contained HTML file with embedded Three.js for interactive 3D visualization:
- Reads binary STL from output directory
- Base64-encodes it inline (no server needed)
- Renders with OrbitControls, metallic material, shadows, grid
- Opens in default browser via `open` command

### C. Infrastructure (`src/infra.rs`)

| Component | Purpose |
|-----------|---------|
| `GitJournal` | Git snapshotting for audit trail |
| `PythonLinter` | Pre-flight syntax validation (`py_compile`) |
| `Fabricator` | Context bundle for legacy operations |
| `apply_search_and_replace` | Robust text substitution |

---

## 5. Review Pipeline with Consensus Tracking

### ReviewConsensus Struct
```rust
pub struct ReviewConsensus {
    pub reviewer: bool,           // Code quality
    pub physics_reviewer: bool,   // Geometry/physics
    pub intent_reviewer: bool,    // User intent
    pub compliance_reviewer: bool, // EU regulations (FINAL GATE)
}

impl ReviewConsensus {
    pub fn approve(&mut self, name: &str); // Mark reviewer approved
    pub fn has_consensus(&self) -> bool;    // All 4 approved?
    pub fn approval_count(&self) -> usize;  // Count of approvals
    pub fn pending_list(&self) -> Vec<&str>; // Who hasn't approved
}
```

### Approval Chain
```
Coder → Reviewer (1/4) → PhysicsReviewer (2/4) → IntentReviewer (3/4) 
     → ComplianceReviewer (4/4 — FINAL GATE)
     → Supervisor (FINALIZE or DELEGATE)
```

### Console Output
```
   ✅ Coder: PythonLinter validated code successfully
   📝 CodeReviewer APPROVED the code quality
   🔄 Consensus: 1/4 reviewers approved
   🔬 PhysicsReviewer APPROVED the physics/geometry
   🔄 Consensus: 2/4 reviewers approved
   🎯 IntentReviewer APPROVED - Meets user intent!
   🔄 Consensus: 3/4 reviewers approved
   ⚖️ ComplianceReviewer APPROVED - EU regulations satisfied!
   ✅ FULL CONSENSUS REACHED: All 4/4 reviewers have approved!
```

### Revision Flow
```
Any Reviewer → REVISE: coder <feedback> → Coder (fixes) → Back to Reviewer chain
Any Reviewer → REVISE: researcher <question> → Researcher (researches) → Coder → ...
```

**Note:** On REVISE, the ReviewConsensus is NOT reset — each reviewer keeps their approval status.

---

## 6. Auto-Execution Pipeline

After the Supervisor finalizes the code:

1. **Code Sanitization** — Strip trailing LLM commentary from the output
2. **Save Script** — Write to `.output/<project>/generated_script_*.py`
3. **Git Commit** — Snapshot the generated script
4. **Execute** — `uv run --with build123d python3 script.py` (auto-resolves packages)
5. **3D Viewer** — If `.stl` files produced, generate `viewer.html` and open in browser
6. **Git Commit** — Snapshot execution output (STEP, STL, SVG files)

---

## 7. Agent Knowledge System

Each agent has access to a **persistent knowledge base** that enables continuous learning.

### Knowledge Tools

| Tool | Purpose | Storage |
|------|---------|---------|
| **KnowledgeBase** | Read domain expertise | `docs/knowledge/<agent>_knowledge.md` |
| **LearnKnowledge** | Save new insights | `docs/knowledge/learned/<agent>_learned.md` |

### How Learning Works

1. **Agent queries KnowledgeBase** for known patterns (prioritized over web search)
2. **Agent uses WebSearcher** for new information not in knowledge base
3. **Agent saves valuable discoveries** via LearnKnowledge tool
4. **Learned content persists** for future workflow runs

---

## 8. Deployment Strategy

| Component | Technology |
|-----------|------------|
| **AI Provider** | Anthropic Claude via Azure MaaS |
| **Model** | `claude-opus-4-5` (configurable) |
| **Web Search** | serper.dev via `serpscraper` crate |
| **Package Mgmt** | `uv` (Python) |
| **3D Preview** | Three.js (CDN, embedded in HTML) |
| **Error Handling** | `thiserror` (library) + `anyhow` (app) |

---

## 9. Structured Output Folder

All workflow artifacts are saved to a structured, Git-tracked folder:

```
.output/
└── my_project_20260228_150907/
    ├── .git/                          # Auto-initialized Git repo
    ├── step_0005_context.json         # Periodic context snapshot
    ├── step_0005_code.py              # Code at step 5
    ├── generated_script_*.py          # Final approved script
    ├── output.step                    # STEP file (CAD interchange)
    ├── output.stl                     # STL file (3D printing)
    └── viewer.html                    # Interactive 3D viewer
```

**API:**
```rust
// With project name
orchestrator.run_with_project(mission, Some("my_project".into())).await?;

// Auto-named with timestamp
orchestrator.run(mission).await?;  // Creates .output/output_20260228_123456/
```

---

## 10. Observability: OpenTelemetry + OTLP

The runtime is instrumented for trace-first debugging and production monitoring.

### Telemetry bootstrap

- Module: `src/telemetry.rs`
- Initializes `tracing` subscriber and OTLP exporter once at startup
- Flushes spans on graceful shutdown

### Span taxonomy

| Layer | Span Name | Purpose |
|------|-----------|---------|
| Request Root | `rig_build123d_request` | One mission objective lifecycle |
| Orchestration | `agent_orchestrator` | End-to-end state machine execution |
| Step | `agent_turn` | One agent turn/transition with step index |
| Tools | `tool.*` | Deterministic visibility into tool execution |
| Smoke Probe | `otel_smoke_probe` | Collector connectivity validation |

### OTLP environment

| Variable | Meaning |
|----------|---------|
| `OTEL_ENABLED` | Enable/disable OTLP export (`true`/`false`) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | Collector endpoint (`http://localhost:4317` default for gRPC; `http://localhost:4318` for HTTP) |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL` / `OTEL_EXPORTER_OTLP_PROTOCOL` | `grpc` (default), `http/protobuf`, `http/json` |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS` / `OTEL_EXPORTER_OTLP_HEADERS` | Cloud auth header pair(s), e.g. ingestion key |
| `OTEL_EXPORTER_OTLP_TRACES_COMPRESSION` / `OTEL_EXPORTER_OTLP_COMPRESSION` | `gzip` or unset/empty for no compression |
| `OTEL_TRACES_SAMPLER_ARG` | Trace sampling ratio `0.0..=1.0` |
| `DEPLOYMENT_ENVIRONMENT` | Environment tag (`development`, `staging`, `production`) |

### Operational checks

1. Run `cargo run --example otel_smoke` with your OTLP vars.
2. Verify trace tree contains `otel_smoke_probe`.
3. Run a real objective and validate hierarchy:
   - `rig_build123d_request`
   - `agent_orchestrator`
   - `agent_turn`
   - relevant `tool.*` spans
