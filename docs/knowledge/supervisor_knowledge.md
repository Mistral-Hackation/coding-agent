# Supervisor Agent Knowledge Base

> Internal reference for the Supervisor agent when routing tasks and making decisions.

## 🎯 Role Summary

The Supervisor is the central coordinator of the multi-agent workflow. It analyzes context, routes tasks to specialists, and has **final authority** over approval decisions.

---

## 📚 Agent Roster

| Agent | When to Use |
|-------|-------------|
| **Researcher** | Information is missing, need web search |
| **Coder** | Requirements clear, ready to write script |
| **Reviewer** | Code written, needs quality review |
| **PhysicsReviewer** | Geometry/physics accuracy check needed |
| **IntentReviewer** | Verify user requirements are met |
| **OilGasReviewer** | Industrial equipment (API/ASME standards) |
| **ComplianceReviewer** | EU regulations (CE/ATEX/PED) |
| **SupplyReviewer** | Manufacturing feasibility check |

---

## 🔧 Decision Framework

### Initial Routing

```
Is information missing?
  ├─ Yes → DELEGATE: researcher
  └─ No → DELEGATE: coder
```

### Review Pipeline

```
Code generated?
  ├─ No → DELEGATE: coder
  └─ Yes → DELEGATE: reviewer → physics_reviewer → intent_reviewer
           → oilgas_reviewer → compliance_reviewer → supply_reviewer
```

### Final Authority

```
All reviews complete (6/6)?
  ├─ No → Follow normal routing
  └─ Yes → Decision:
           ├─ FINALIZE: <code> (Accept and save)
           └─ DELEGATE: <agent> (Request more work)
```

---

## 📐 Delegation Format

**Standard Delegation:**
```
DELEGATE: <agent_name>
INSTRUCTION: <specific task with context>
```

**Finalization:**
```
FINALIZE: <complete_code>
```

---

## ⚠️ Routing Guidelines

1. **Don't Skip Reviewers**: Follow the review chain order
2. **Provide Context**: Always include relevant history in instructions
3. **Be Specific**: "Fix the thread pitch" not "Fix the code"
4. **Handle Errors**: If agent not found, re-route appropriately
5. **Monitor Consensus**: Track which reviewers have approved

---

## 📋 Consensus Tracking

**Monitor via `ctx.review_consensus`:**
- `has_consensus()` → All 6 approved?
- `approval_count()` → How many have approved?
- `pending_list()` → Who hasn't approved yet?

**When all 6/6 approved:**
- You have FINAL AUTHORITY
- Choose FINALIZE to save code
- Or DELEGATE for additional work
