use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The PhysicsReviewer agent evaluates scripts from a physics and geometry perspective.
///
/// It reviews the generated code for realistic physics, proper geometric transformations,
/// and mathematical accuracy in 3D space.
pub struct PhysicsReviewer {
    inner: ClaudeAgent,
}

impl PhysicsReviewer {
    /// Creates a new PhysicsReviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::PHYSICS_REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "physics_reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "physics_reviewer",
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
impl Specialist for PhysicsReviewer {
    fn name(&self) -> &str {
        "physics_reviewer"
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
            "Objective: {}\n\nCode to review for physics/geometry:\n{}\n\nReview this build123d Python CAD code for physical and geometric accuracy.",
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
            println!("   🔬 PhysicsReviewer APPROVED the physics/geometry");
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("physics_reviewer");
            println!(
                "   🔄 Consensus: {}/4 reviewers approved",
                ctx.review_consensus.approval_count()
            );
            // Pass to next reviewer (intent_reviewer)
            ctx.conversation_history
                .push("PhysicsReviewer: Physics/geometry approved".to_string());
            Ok(TurnResult::Delegate {
                target_agent: "intent_reviewer".to_string(),
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
                "   🔬 PhysicsReviewer requests PHYSICS REVISIONS from [{}]",
                target
            );
            ctx.conversation_history.push(format!(
                "PhysicsReviewer: Physics corrections needed - {}",
                feedback
            ));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            ctx.conversation_history
                .push(format!("PhysicsReviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
