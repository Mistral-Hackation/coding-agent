use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The IntentReviewer agent evaluates scripts based on user's original intention.
///
/// It reviews the generated code to ensure it meets the user's actual needs
/// and follows industry best practices.
pub struct IntentReviewer {
    inner: ClaudeAgent,
}

impl IntentReviewer {
    /// Creates a new IntentReviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::INTENT_REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "intent_reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "intent_reviewer",
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
impl Specialist for IntentReviewer {
    fn name(&self) -> &str {
        "intent_reviewer"
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
            "ORIGINAL USER OBJECTIVE: {}\n\nCode to review:\n{}\n\nDoes this code fully satisfy the user's original intent and follow industry standards?",
            ctx.objective, code
        );

        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        if response.contains("APPROVED:") {
            let (_, code) = response.split_once("APPROVED:").ok_or_else(|| {
                AgentError::ContextError("Parsed 'APPROVED:' key but failed to split string".into())
            })?;
            println!("   🎯 IntentReviewer APPROVED - Meets user intent!");
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("intent_reviewer");
            println!(
                "   🔄 Consensus: {}/4 reviewers approved",
                ctx.review_consensus.approval_count()
            );
            // Pass to Compliance reviewer (final review stage)
            ctx.conversation_history
                .push("IntentReviewer: User intent approved".to_string());
            Ok(TurnResult::Delegate {
                target_agent: "compliance_reviewer".to_string(),
                instruction: code.trim().to_string(),
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
                "   🎯 IntentReviewer requests INTENT REVISIONS from [{}]",
                target
            );
            ctx.conversation_history.push(format!(
                "IntentReviewer: Does not meet user intent - {}",
                feedback
            ));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            ctx.conversation_history
                .push(format!("IntentReviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
