use super::utils::parse_delegation;
use crate::agentic::errors::AgentError;
use crate::agentic::{GlobalContext, Specialist, TurnResult};
use crate::types::ClaudeAgent;
use async_trait::async_trait;
use rig::agent::AgentBuilder;
use rig::completion::Prompt;

/// The Coder agent is a build123d CAD Python expert.
///
/// It receives instructions (usually from the Researcher) and writes the final build123d Python script.
pub struct Coder {
    inner: ClaudeAgent,
}

impl Coder {
    /// Creates a new Coder agent with the given model.
    pub fn new(model: rig::providers::anthropic::completion::CompletionModel) -> Self {
        let preamble = crate::agentic::agents::prompts::CODER_PREAMBLE;

        let agent = AgentBuilder::new(model)
            .tool(crate::agentic::tools::knowledge_base::KnowledgeBase::new(
                "coder",
            ))
            .tool(crate::agentic::tools::knowledge_base::LearnKnowledge::new(
                "coder",
            ))
            .tool(crate::agentic::tools::build123d_docs::Build123dDocsReader::new())
            .tool(crate::agentic::tools::example_searcher::ExampleSearcher::new())
            .tool(crate::agentic::tools::system::LinterTool)
            .tool(crate::agentic::tools::system::EditTool)
            .tool(crate::agentic::tools::code_editor::CodeSnippetSearch::new())
            .tool(crate::agentic::tools::code_editor::CodeSnippetReplace::new())
            .preamble(preamble)
            .max_tokens(8192) // Allow longer code outputs
            .build();
        Self { inner: agent }
    }
}

#[async_trait]
impl Specialist for Coder {
    fn name(&self) -> &str {
        "coder"
    }

    async fn run_turn(&self, mut ctx: GlobalContext) -> Result<TurnResult, AgentError> {
        let prompt = format!(
            "Objective: {}\nHistory: {:?}\nWrite the code.",
            ctx.objective, ctx.conversation_history
        );

        // Use multi_turn to allow tool calls (KnowledgeBase) to complete
        let response = self
            .inner
            .prompt(&prompt)
            .multi_turn(100)
            .await
            .map_err(AgentError::from)?;

        // Extract code from response (look for code blocks or the whole response)
        let code = if response.contains("```python") {
            // Extract code between ```python and ```
            response
                .split("```python")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .unwrap_or(&response)
                .trim()
                .to_string()
        } else if response.contains("```") {
            // Extract code between ``` and ```
            response
                .split("```")
                .nth(1)
                .unwrap_or(&response)
                .trim()
                .to_string()
        } else {
            response.clone()
        };

        // Always add the code to conversation history so reviewers can see it
        ctx.conversation_history
            .push(format!("GENERATED_CODE:\n{}", code));

        // 🛡️ PROGRAMMATIC LINTER ENFORCEMENT
        // Validate the code with PythonLinter before any delegation
        let linter = crate::infra::PythonLinter;
        match linter.validate(&code) {
            Ok(_) => {
                println!("   ✅ Coder: PythonLinter validated code successfully");
            }
            Err(lint_error) => {
                // Code has syntax errors - do NOT delegate, keep working
                println!(
                    "   ⚠️ Coder: PythonLinter found syntax errors:\n{}",
                    lint_error
                );
                ctx.conversation_history.push(format!(
                    "LINT_ERROR: The generated code has syntax errors:\n{}",
                    lint_error
                ));
                // Return KeepWorking so the agent can fix the code
                return Ok(TurnResult::KeepWorking {
                    thought: format!(
                        "My code has syntax errors. I need to fix them:\n{}",
                        lint_error
                    ),
                    new_context: ctx,
                });
            }
        }

        // Enforce required exports before delegating to reviewers.
        let has_step = code.contains("export_step(");
        let has_stl = code.contains("export_stl(");
        let has_svg = code.contains("export_svg(");
        if !(has_step && has_stl && has_svg) {
            let mut missing = Vec::new();
            if !has_step {
                missing.push("export_step");
            }
            if !has_stl {
                missing.push("export_stl");
            }
            if !has_svg {
                missing.push("export_svg");
            }

            let missing_msg = format!("missing export calls: {}", missing.join(", "));
            ctx.conversation_history
                .push(format!("EXPORT_VALIDATION: {}", missing_msg));

            return Ok(TurnResult::KeepWorking {
                thought: format!(
                    "I need to fix the code to include all required exports ({missing_msg})."
                ),
                new_context: ctx,
            });
        }

        if response.contains("DELEGATE:") {
            let (target, _instruction) = parse_delegation(&response)?;
            Ok(TurnResult::Delegate {
                target_agent: target,
                instruction: format!("Review this code:\n{}", code),
                new_context: ctx,
            })
        } else if response.contains("FINAL:") {
            Ok(TurnResult::Delegate {
                target_agent: "reviewer".to_string(),
                instruction: format!("Review this code:\n{}", code),
                new_context: ctx,
            })
        } else {
            // Default: Send to reviewer for approval
            Ok(TurnResult::Delegate {
                target_agent: "reviewer".to_string(),
                instruction: format!("Review this code:\n{}", code),
                new_context: ctx,
            })
        }
    }
}
