use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The ComplianceReviewer agent checks designs for European regulatory compliance.
///
/// It reviews designs for CE marking, ATEX, PED, REACH, RoHS, and other EU directives.
pub struct ComplianceReviewer {
    inner: ClaudeAgent,
}

impl ComplianceReviewer {
    /// Creates a new ComplianceReviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::COMPLIANCE_REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "compliance_reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "compliance_reviewer",
            ))
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .tool(crate::agentic::tools::system::LinterTool)
            .preamble(preamble)
            .max_tokens(4096)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for ComplianceReviewer {
    fn name(&self) -> &str {
        "compliance_reviewer"
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
            "ORIGINAL USER OBJECTIVE: {}\n\nDesign/Code to review for EU regulatory compliance:\n{}\n\nCheck if this design meets European regulatory requirements.",
            ctx.objective, code
        );

        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        // Normalize the response to detect approval robustly.
        // LLMs may emit "APPROVED:", "✅ APPROVED", "## APPROVED", etc.
        let normalized = response.replace("✅", "").replace('#', "");
        let is_approved =
            normalized.contains("APPROVED:") || normalized.to_uppercase().contains("APPROVED");

        if is_approved {
            // Try to extract code from after "APPROVED:" if present, otherwise
            // fall back to the code we already have from conversation history.
            let approved_code = response
                .split_once("APPROVED:")
                .map(|(_, c)| c.trim().to_string())
                .filter(|c| !c.is_empty())
                .unwrap_or_else(|| code.to_string());

            println!("   ⚖️ ComplianceReviewer APPROVED - EU regulations satisfied!");
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("compliance_reviewer");
            println!(
                "   🔄 Consensus: {}/4 reviewers approved",
                ctx.review_consensus.approval_count()
            );
            // ComplianceReviewer is the FINAL GATE
            ctx.conversation_history
                .push("ComplianceReviewer: EU compliance approved".to_string());

            // Check if full consensus has been reached
            if ctx.review_consensus.has_consensus() {
                println!("   ✅ FULL CONSENSUS REACHED: All 4/4 reviewers have approved!");
                Ok(TurnResult::Delegate {
                    target_agent: "supervisor".to_string(),
                    instruction: format!(
                        "ALL_REVIEWS_COMPLETE: All 4 reviewers have approved. The code passed: \
                         Reviewer, PhysicsReviewer, IntentReviewer, ComplianceReviewer. \
                         You have FINAL AUTHORITY. Choose:\n\
                         1. FINAL: <the code> — Accept and end the mission\n\
                         2. DELEGATE: coder <feedback> — Request changes\n\n\
                         Code:\n{}",
                        approved_code
                    ),
                    new_context: ctx,
                })
            } else {
                println!(
                    "   🔄 Consensus: {}/4 reviewers approved (pending: {:?})",
                    ctx.review_consensus.approval_count(),
                    ctx.review_consensus.pending_list()
                );
                Ok(TurnResult::Delegate {
                    target_agent: "supervisor".to_string(),
                    instruction: format!(
                        "PARTIAL_CONSENSUS: {}/4 reviews done. Pending: {:?}. Route to next pending reviewer.",
                        ctx.review_consensus.approval_count(),
                        ctx.review_consensus.pending_list()
                    ),
                    new_context: ctx,
                })
            }
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
                "   ⚖️ ComplianceReviewer requests COMPLIANCE REVISIONS from [{}]",
                target
            );
            ctx.conversation_history.push(format!(
                "ComplianceReviewer: Regulatory issue - {}",
                feedback
            ));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            ctx.conversation_history
                .push(format!("ComplianceReviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
