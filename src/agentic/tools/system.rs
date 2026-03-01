use crate::infra::{GitJournal, PythonLinter, apply_search_and_replace};
use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// --- Git Tool ---

/// Arguments for the Git tool.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct GitArgs {
    /// The commit message describing the changes.
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Git error: {0}")]
/// Error type for Git operations.
pub struct GitError(String);

/// Tool to create snapshots of the codebase.
pub struct GitTool {
    journal: Arc<Mutex<GitJournal>>,
    actor: String,
}

impl GitTool {
    /// Creates a new `GitTool` instance.
    pub fn new(journal: Arc<Mutex<GitJournal>>, actor: &str) -> Self {
        Self {
            journal,
            actor: actor.to_string(),
        }
    }
}

impl Tool for GitTool {
    const NAME: &'static str = "GitJournal";
    type Error = GitError;
    type Args = GitArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(GitArgs);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "GitJournal".to_string(),
            description: "Creates a git commit snapshot. Use BEFORE risky changes or AFTER completing a step.".to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.git_snapshot",
            tool_name = Self::NAME,
            actor = %self.actor
        );
        span.record_model_input(&serde_json::json!({
            "message_preview": truncate_for_telemetry(&args.message, 180),
            "message_len": args.message.len(),
        }));
        let _guard = span.enter();

        let journal = self.journal.lock().map_err(|e| GitError(e.to_string()))?;
        journal.snapshot(&self.actor, &args.message);
        span.record_model_output(&serde_json::json!({
            "status": "snapshot_created",
        }));
        Ok(format!("Snapshot created: '{}'", args.message))
    }
}

// --- Linter Tool ---

/// Arguments for the Python Linter tool.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct LinterArgs {
    /// The Python code to validate.
    pub code: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Linter error: {0}")]
/// Error type for Linter operations.
pub struct LinterError(String);

/// Tool to validate Python code syntax.
#[derive(Serialize, Deserialize)]
pub struct LinterTool;

impl Tool for LinterTool {
    const NAME: &'static str = "PythonLinter";
    type Error = LinterError;
    type Args = LinterArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(LinterArgs);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "PythonLinter".to_string(),
            description: "Validates Python syntax. Returns 'VALID' or error. YOU MUST USE THIS before submitting code.".to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.python_linter",
            tool_name = Self::NAME,
            code_len = args.code.len()
        );
        span.record_model_input(&serde_json::json!({
            "code_preview": truncate_for_telemetry(&args.code, 220),
            "code_len": args.code.len(),
        }));
        let _guard = span.enter();

        let linter = PythonLinter;
        match linter.validate(&args.code) {
            Ok(_) => {
                span.record_model_output(&serde_json::json!({
                    "status": "valid",
                }));
                Ok("VALID".to_string())
            }
            Err(e) => {
                span.record_model_output(&serde_json::json!({
                    "status": "syntax_error",
                    "error_preview": truncate_for_telemetry(&e, 220),
                }));
                Ok(format!("SYNTAX ERROR:\n{}", e))
            }
        }
    }
}

// --- Edit Tool (Search & Replace) ---

/// Arguments for the Edit tool.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct EditArgs {
    /// The exact block of code to find.
    pub search: String,
    /// The code to replace it with.
    pub replace: String,
    /// The full content of the file/code to edit.
    pub full_code: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Edit error: {0}")]
/// Error type for Edit operations.
pub struct EditError(String);

#[derive(Serialize, Deserialize)]
/// Tool for performing search and replace operations on code.
pub struct EditTool;

impl Tool for EditTool {
    const NAME: &'static str = "SearchAndReplace";
    type Error = EditError;
    type Args = EditArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(EditArgs);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "SearchAndReplace".to_string(),
            description: "Exact string search and replace. Use to apply precise patches to code."
                .to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.search_and_replace",
            tool_name = Self::NAME,
            full_code_len = args.full_code.len()
        );
        span.record_model_input(&serde_json::json!({
            "search_preview": truncate_for_telemetry(&args.search, 180),
            "replace_preview": truncate_for_telemetry(&args.replace, 180),
            "full_code_len": args.full_code.len(),
        }));
        let _guard = span.enter();

        let output = apply_search_and_replace(&args.full_code, &args.search, &args.replace)
            .map_err(EditError);
        match &output {
            Ok(updated) => span.record_model_output(&serde_json::json!({
                "status": "updated",
                "updated_len": updated.len(),
            })),
            Err(error) => span.record_model_output(&serde_json::json!({
                "status": "error",
                "error": error.to_string(),
            })),
        }
        output
    }
}

fn truncate_for_telemetry(input: &str, max_chars: usize) -> String {
    let mut preview: String = input.chars().take(max_chars).collect();
    if input.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}
