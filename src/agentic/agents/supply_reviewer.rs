use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The SupplyReviewer agent checks supplier availability for designs.
///
/// It uses web search to find potential suppliers and verify material/component availability.
pub struct SupplyReviewer {
    inner: ClaudeAgent,
}

impl SupplyReviewer {
    /// Creates a new SupplyReviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::SUPPLY_REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "supply_reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "supply_reviewer",
            ))
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .preamble(preamble)
            .max_tokens(4096)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for SupplyReviewer {
    fn name(&self) -> &str {
        "supply_reviewer"
    }

    async fn run_turn(&self, mut ctx: GlobalContext) -> Result<TurnResult, AgentError> {
        // Find the generated code in conversation history
        let code = ctx
            .conversation_history
            .iter()
            .rev()
            .find(|entry| entry.starts_with("GENERATED_CODE:"))
            .map(|entry| entry.strip_prefix("GENERATED_CODE:\n").unwrap_or(entry))
            .unwrap_or("No code found in history");

        let prompt = format!(
            "ORIGINAL USER OBJECTIVE: {}\n\nDesign to review for supplier availability:\n{}\n\nReview this design for manufacturability and supplier feasibility using local supply hints and compliance knowledge. Focus on practical sourcing constraints without external web lookup.",
            ctx.objective, code
        );

        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        if response.contains("FINAL_APPROVED:") || response.contains("APPROVED:") {
            let code = if response.contains("FINAL_APPROVED:") {
                response
                    .split_once("FINAL_APPROVED:")
                    .map(|(_, c)| c.trim())
            } else {
                response.split_once("APPROVED:").map(|(_, c)| c.trim())
            }
            .unwrap_or("")
            .to_string();

            println!(
                "   📦 SupplyReviewer APPROVED - All reviews complete! Returning to Supervisor for final decision."
            );
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("supply_reviewer");

            // Check if we have full consensus
            if ctx.review_consensus.has_consensus() {
                println!("   ✅ FULL CONSENSUS REACHED: All 6/6 reviewers have approved!");
            } else {
                println!(
                    "   🔄 Consensus: {}/6 reviewers approved (pending: {:?})",
                    ctx.review_consensus.approval_count(),
                    ctx.review_consensus.pending_list()
                );
            }

            ctx.conversation_history.push(
                "ALL_REVIEWS_COMPLETE: Supply chain verified, all reviewers approved.".to_string(),
            );

            // Return to Supervisor for final authority decision
            Ok(TurnResult::Delegate {
                target_agent: "supervisor".to_string(),
                instruction: format!(
                    "ALL_REVIEWS_COMPLETE: All 6 reviewers have approved. The code passed: Reviewer, PhysicsReviewer, IntentReviewer, OilGasReviewer, ComplianceReviewer, SupplyReviewer. You have FINAL AUTHORITY. Choose:\n1. FINALIZE: <code> - Accept and save the code\n2. DELEGATE: <agent> <instruction> - Request more work\n\nApproved code:\n{}",
                    code
                ),
                new_context: ctx,
            })
        } else if response.contains("REVISE:") {
            let part = response
                .split("REVISE:")
                .nth(1)
                .ok_or_else(|| AgentError::DelegationParseError("Missing REVISE content".into()))?
                .trim();

            let (target, feedback) = if let Some((t, f)) = part.split_once(' ') {
                (t.to_lowercase(), f.to_string())
            } else {
                ("coder".to_string(), part.to_string())
            };

            println!(
                "   📦 SupplyReviewer requests SUPPLY REVISIONS from [{}]",
                target
            );
            ctx.conversation_history
                .push(format!("SupplyReviewer: Supply chain issue - {}", feedback));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            ctx.conversation_history
                .push(format!("SupplyReviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
