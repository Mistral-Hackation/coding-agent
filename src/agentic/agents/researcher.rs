use super::utils::parse_delegation;
use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The Researcher agent is responsible for gathering information.
///
/// It relies on local knowledge and example lookup tools.
/// It delegates to the `Coder` once sufficient information has been gathered.
pub struct Researcher {
    inner: ClaudeAgent,
}

impl Researcher {
    /// Creates a new Researcher agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::RESEARCHER_PREAMBLE.to_string();

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "researcher",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "researcher",
            ))
            .tool(crate::agentic::tools::example_searcher::ExampleSearcher::new())
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .preamble(&preamble)
            .max_tokens(4096)
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for Researcher {
    fn name(&self) -> &str {
        "researcher"
    }

    async fn run_turn(&self, mut ctx: GlobalContext) -> Result<TurnResult, AgentError> {
        let prompt = format!(
            "Objective: {}\nHistory: {:?}\nArtifacts: {:?}",
            ctx.objective, ctx.conversation_history, ctx.artifacts
        );

        // 1. Run Rig (Agent autonomously calls tools here)
        // Use .multi_turn(5) to allow up to 5 tool call iterations
        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        // 2. Parse Intent (Dynamic Routing Logic)
        if response.contains("DELEGATE:") {
            // Parse "DELEGATE: coder Write the script"
            let (target, instruction) = parse_delegation(&response)?;

            ctx.conversation_history
                .push(format!("Researcher: Handing off to {}", target));

            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction,
                new_context: ctx,
            })
        } else if response.contains("THINKING:") {
            ctx.conversation_history
                .push(format!("Researcher: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        } else {
            // Fallback/Default behavior
            ctx.conversation_history
                .push(format!("Researcher: {}", response));
            Ok(TurnResult::KeepWorking {
                thought: response,
                new_context: ctx,
            })
        }
    }
}
