use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The OilGasReviewer agent evaluates scripts from an oil & gas industry perspective.
///
/// It specializes in high pressure pump systems, industrial equipment design,
/// and oil & gas industry standards for build123d CAD models.
pub struct OilGasReviewer {
    inner: ClaudeAgent,
}

impl OilGasReviewer {
    /// Creates a new OilGasReviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::OIL_GAS_REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "oilgas_reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "oilgas_reviewer",
            ))
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .preamble(preamble)
            .max_tokens(4096)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for OilGasReviewer {
    fn name(&self) -> &str {
        "oilgas_reviewer"
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
            "ORIGINAL USER OBJECTIVE: {}\n\nCode to review for oil & gas industry standards:\n{}\n\nDoes this code meet industrial quality standards for high pressure pump/equipment visualization?",
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
            println!("   🛢️ OilGasReviewer APPROVED - Meets industry standards!");
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("oilgas_reviewer");
            println!(
                "   🔄 Consensus: {}/5 reviewers approved",
                ctx.review_consensus.approval_count()
            );
            // Pass to Compliance reviewer
            ctx.conversation_history
                .push("OilGasReviewer: Industry standards approved".to_string());
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
                "   🛢️ OilGasReviewer requests INDUSTRIAL REVISIONS from [{}]",
                target
            );
            ctx.conversation_history.push(format!(
                "OilGasReviewer: Industrial corrections needed - {}",
                feedback
            ));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            ctx.conversation_history
                .push(format!("OilGasReviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
