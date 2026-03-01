use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The Supervisor agent acts as the reasoning engine and router.
///
/// It analyzes the high-level objective and current state to decide which specialist should act next.
/// The Supervisor does **not** write code or search the web; it purely manages the workflow.
pub struct Supervisor {
    inner: ClaudeAgent,
}

impl Supervisor {
    /// Creates a new Supervisor agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let agent = AgentBuilder::new(model)
            .preamble(crate::agentic::agents::prompts::SUPERVISOR_PREAMBLE)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for Supervisor {
    fn name(&self) -> &str {
        "supervisor"
    }

    async fn run_turn(&self, ctx: GlobalContext) -> Result<TurnResult, AgentError> {
        // Check if all reviews are complete using the consensus tracker
        let has_consensus = ctx.review_consensus.has_consensus();
        let approval_count = ctx.review_consensus.approval_count();

        println!(
            "   📊 Consensus Status: {}/4 reviewers approved",
            approval_count
        );
        if !has_consensus && approval_count > 0 {
            println!("   ⏳ Pending: {:?}", ctx.review_consensus.pending_list());
        }

        // Build a comprehensive prompt that asks the LLM to decide AND explain why
        let recent_history: Vec<String> = ctx
            .conversation_history
            .iter()
            .rev()
            .take(5)
            .cloned()
            .collect();

        let prompt = if has_consensus {
            // Special prompt when all reviews are complete
            format!(
                r#"You are the Supervisor of a multi-agent build123d CAD code generation system.
You have FINAL AUTHORITY over the workflow.

OBJECTIVE: {}

STATUS: ALL REVIEWS COMPLETE! 
All 4 reviewers (Reviewer, PhysicsReviewer, IntentReviewer, ComplianceReviewer) have APPROVED the code.

EXECUTION CONTEXT:
The final script is standalone Python using `from build123d import *`.
It will be run as: `python3 <generated_script.py>`
Output formats: STEP (.step) for CAD interchange, STL (.stl) for 3D printing.

RECENT HISTORY:
{}

YOUR DECISION OPTIONS:
1. FINALIZE: <complete_code> - Accept and save the final code. Workflow ends.
2. DELEGATE: <agent_name>
   INSTRUCTION: <what more work is needed>

If the code meets all requirements and quality standards, FINALIZE it.
If you see issues or want improvements, DELEGATE back to the appropriate agent.

Make your decision:"#,
                ctx.objective,
                recent_history.join("\n")
            )
        } else {
            // Normal routing prompt
            format!(
                r#"You are the Supervisor of a multi-agent build123d CAD code generation system.

OBJECTIVE: {}

EXECUTION CONTEXT:
The final script is standalone Python using `from build123d import *`.
It will be run as: `python3 <generated_script.py>`
Output formats: STEP (.step) for CAD interchange, STL (.stl) for 3D printing.

CONVERSATION HISTORY (recent):
{}

AVAILABLE AGENTS:
- researcher: Gathers information from local docs and examples (use when info is missing)
- coder: Writes build123d Python CAD scripts (use when requirements are clear)
- reviewer: Reviews code quality (use after code is written)
- physics_reviewer: Reviews geometry/topology accuracy (use after code review)
- intent_reviewer: Verifies user intent is met (use after physics review)
- compliance_reviewer: European regulatory compliance / final gate (use as final review stage)

TASK: Analyze the state and decide:
1. Which agent should act next?
2. What SPECIFIC instruction should they receive?

FORMAT YOUR RESPONSE AS:
DELEGATE: <agent_name>
INSTRUCTION: <detailed instruction based on current context>

Be specific! Don't just say "proceed" - explain exactly what they need to do."#,
                ctx.objective,
                recent_history.join("\n")
            )
        };

        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        // Check for FINALIZE signal (Supervisor's final authority)
        if response.contains("FINALIZE:") {
            let code = response
                .split("FINALIZE:")
                .nth(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| {
                    // Try to extract code from conversation history
                    ctx.conversation_history
                        .iter()
                        .rev()
                        .find(|entry| entry.starts_with("GENERATED_CODE:"))
                        .map(|entry| {
                            entry
                                .strip_prefix("GENERATED_CODE:\n")
                                .unwrap_or(entry)
                                .to_string()
                        })
                        .unwrap_or_default()
                });

            println!("   ✅ Supervisor: FINALIZED - All reviews approved, code accepted!");
            return Ok(TurnResult::FinalResult(code));
        }

        // Parse the structured response for delegation
        let target = if let Some(delegate_line) = response.lines().find(|l| l.contains("DELEGATE:"))
        {
            delegate_line
                .split("DELEGATE:")
                .nth(1)
                .map(|s| s.trim().to_lowercase())
                .unwrap_or_else(|| "researcher".to_string())
        } else {
            // Fallback: extract first word that looks like an agent name
            response
                .split_whitespace()
                .find(|word| {
                    [
                        "researcher",
                        "coder",
                        "reviewer",
                        "physics_reviewer",
                        "intent_reviewer",
                        "compliance_reviewer",
                    ]
                    .contains(&word.to_lowercase().as_str())
                })
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "researcher".to_string())
        };

        let instruction =
            if let Some(inst_line) = response.lines().find(|l| l.contains("INSTRUCTION:")) {
                inst_line
                    .split("INSTRUCTION:")
                    .nth(1)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| format!("Work on the objective: {}", ctx.objective))
            } else {
                // Use the full response as context if no structured format
                format!(
                    "Context from supervisor: {}",
                    response.chars().take(500).collect::<String>()
                )
            };

        println!("   📋 Supervisor analysis: Delegating to [{}]", target);

        Ok(TurnResult::Delegate {
            target_agent: target,
            instruction,
            new_context: ctx,
        })
    }
}
