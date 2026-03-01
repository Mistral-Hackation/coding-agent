use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt; // Use standard prompt

/// The Reviewer agent evaluates the Coder's output for quality.
///
/// It reviews the generated code and either approves it (returns FinalResult)
/// or requests revisions by delegating back to Coder or Researcher.
pub struct Reviewer {
    inner: ClaudeAgent,
}

impl Reviewer {
    /// Creates a new Reviewer agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::REVIEWER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "reviewer",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "reviewer",
            ))
            .tool(crate::agentic::tools::example_searcher::ExampleSearcher::new())
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .tool(crate::agentic::tools::code_editor::CodeSnippetSearch::new())
            .tool(crate::agentic::tools::system::LinterTool)
            .preamble(preamble)
            .max_tokens(4096)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for Reviewer {
    fn name(&self) -> &str {
        "reviewer"
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
            "Objective: {}\n\nCode to review:\n{}\n\nReview this build123d Python CAD code for correctness, completeness, and best practices.",
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
            println!("   📝 CodeReviewer APPROVED the code quality");
            // Mark this reviewer as approved in the consensus tracker
            ctx.review_consensus.approve("reviewer");
            println!(
                "   🔄 Consensus: {}/4 reviewers approved",
                ctx.review_consensus.approval_count()
            );
            // Pass to physics reviewer for next stage
            ctx.conversation_history
                .push("CodeReviewer: Code quality approved".to_string());
            Ok(TurnResult::Delegate {
                target_agent: "physics_reviewer".to_string(),
                instruction: code.trim().to_string(),
                new_context: ctx,
            })
        } else if response.contains("REVISE:") {
            // Parse "REVISE: coder <feedback>" or "REVISE: researcher <question>"
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

            println!("   🔄 Reviewer requests REVISIONS from [{}]", target);
            ctx.conversation_history
                .push(format!("Reviewer: Revisions needed - {}", feedback));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: feedback,
                new_context: ctx,
            })
        } else {
            // Default: keep thinking
            ctx.conversation_history
                .push(format!("Reviewer: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
