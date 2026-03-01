//! Code editing tools — agents can search and replace snippets in generated Python code.
//!
//! These tools allow agents to iteratively refine code instead of rewriting
//! from scratch. Useful when a reviewer sends code back for revisions.
//!
//! - [`crate::agentic::tools::code_editor::CodeSnippetSearch`]:
//!   find a pattern/keyword in the current generated code
//! - [`crate::agentic::tools::code_editor::CodeSnippetReplace`]:
//!   search-and-replace a block of code

use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Shared state: the working code file path
// ---------------------------------------------------------------------------

/// Code files are stored in the output directory.
/// Both tools operate on the latest generated script file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeFilePath {
    /// Absolute path to the Python file being edited.
    path: PathBuf,
}

impl CodeFilePath {
    /// Creates a new code file path reference.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

// ============================= TOOL 1: Search =============================

/// Arguments for searching within the generated Python code.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct CodeSearchArgs {
    /// The text pattern to search for in the generated code.
    /// Can be a function name, variable, import, or any code snippet.
    /// Case-insensitive search is performed.
    pub pattern: String,

    /// If true, shows surrounding context lines (3 before + 3 after).
    /// Default: true.
    pub show_context: Option<bool>,
}

/// Error type for CodeSnippetSearch operations.
#[derive(Debug)]
pub struct CodeSearchError(String);

impl std::fmt::Display for CodeSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CodeSearchError {}

/// Searches for a text pattern within the generated Python code.
///
/// Returns matching lines with line numbers and optional context.
/// Agents use this to locate specific code sections before editing.
#[derive(Serialize, Deserialize, Clone)]
pub struct CodeSnippetSearch {
    /// Directory where generated scripts are stored.
    output_dir: PathBuf,
}

impl CodeSnippetSearch {
    /// Create a new CodeSnippetSearch for the default output directory.
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from(".output"),
        }
    }

    /// Create with a custom output directory.
    #[allow(dead_code)]
    pub fn with_dir(dir: PathBuf) -> Self {
        Self { output_dir: dir }
    }
}

impl Default for CodeSnippetSearch {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for CodeSnippetSearch {
    const NAME: &'static str = "CodeSnippetSearch";
    type Error = CodeSearchError;
    type Args = CodeSearchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(CodeSearchArgs);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);

        ToolDefinition {
            name: "CodeSnippetSearch".to_string(),
            description: "Search for a text pattern in the current generated Python code. \
                 Returns matching lines with line numbers and surrounding context. \
                 Use before CodeSnippetReplace to verify exact text to replace."
                .to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.code_snippet_search",
            tool_name = Self::NAME,
            pattern_len = args.pattern.len()
        );
        span.record_model_input(&serde_json::json!({
            "pattern_preview": truncate_for_telemetry(&args.pattern, 180),
            "pattern_len": args.pattern.len(),
            "show_context": args.show_context.unwrap_or(true),
        }));
        let _guard = span.enter();

        let show_context = args.show_context.unwrap_or(true);

        println!(
            "   🔎 Tool Call: CodeSnippetSearch pattern='{}'",
            args.pattern
        );

        // Find the latest generated script in any output subdirectory
        let script_path = find_latest_script(&self.output_dir).ok_or_else(|| {
            CodeSearchError("No generated script found in output directory.".into())
        })?;

        let code = std::fs::read_to_string(&script_path)
            .map_err(|e| CodeSearchError(format!("Failed to read script: {}", e)))?;

        let pattern_lower = args.pattern.to_lowercase();
        let lines: Vec<&str> = code.lines().collect();
        let mut results = Vec::new();

        for (line_number_idx, line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&pattern_lower) {
                let line_number = line_number_idx + 1;

                if show_context {
                    // Show 3 lines before + match + 3 lines after
                    let context_start = line_number_idx.saturating_sub(3);
                    let context_end = (line_number_idx + 4).min(lines.len());

                    let context_block: Vec<String> = lines[context_start..context_end]
                        .iter()
                        .enumerate()
                        .map(|(i, l)| {
                            let actual_line = context_start + i + 1;
                            let marker = if actual_line == line_number {
                                ">>>"
                            } else {
                                "   "
                            };
                            format!("{} {:3}: {}", marker, actual_line, l)
                        })
                        .collect();

                    results.push(context_block.join("\n"));
                } else {
                    results.push(format!("L{}: {}", line_number, line));
                }
            }
        }

        if results.is_empty() {
            let output = format!(
                "Pattern '{}' not found in generated code ({}).",
                args.pattern,
                script_path.display()
            );
            span.record_model_output(&serde_json::json!({
                "status": "not_found",
                "result_len": output.len(),
            }));
            Ok(output)
        } else {
            let output = format!(
                "Found {} match(es) for '{}' in {}:\n\n{}",
                results.len(),
                args.pattern,
                script_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
                results.join("\n---\n")
            );
            span.record_model_output(&serde_json::json!({
                "status": "ok",
                "matches": results.len(),
                "result_len": output.len(),
            }));
            Ok(output)
        }
    }
}

// =========================== TOOL 2: Replace =============================

/// Arguments for search-and-replace within the generated Python code.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct CodeReplaceArgs {
    /// The exact code block to search for (must match exactly, including whitespace).
    /// Use CodeSnippetSearch first to find the exact text.
    pub search_block: String,

    /// The replacement code block. This will replace the search_block entirely.
    pub replace_block: String,
}

/// Error type for CodeSnippetReplace operations.
#[derive(Debug)]
pub struct CodeReplaceError(String);

impl std::fmt::Display for CodeReplaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CodeReplaceError {}

/// Performs search-and-replace on the generated Python code.
///
/// The agent must provide the exact code block to find and the replacement.
/// Uses `apply_search_and_replace` from the infra module for safe, atomic edits.
///
/// Always use [`CodeSnippetSearch`] first to verify the exact text before replacing.
#[derive(Serialize, Deserialize, Clone)]
pub struct CodeSnippetReplace {
    /// Directory where generated scripts are stored.
    output_dir: PathBuf,
}

impl CodeSnippetReplace {
    /// Create a new CodeSnippetReplace for the default output directory.
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from(".output"),
        }
    }

    /// Create with a custom output directory.
    #[allow(dead_code)]
    pub fn with_dir(dir: PathBuf) -> Self {
        Self { output_dir: dir }
    }
}

impl Default for CodeSnippetReplace {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for CodeSnippetReplace {
    const NAME: &'static str = "CodeSnippetReplace";
    type Error = CodeReplaceError;
    type Args = CodeReplaceArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(CodeReplaceArgs);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);

        ToolDefinition {
            name: "CodeSnippetReplace".to_string(),
            description: "Perform search-and-replace on the current generated Python code. \
                 Provide the EXACT code block to find (use CodeSnippetSearch first) \
                 and the replacement code. The edit is atomic — if the search block \
                 is not found exactly, the operation fails safely."
                .to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.code_snippet_replace",
            tool_name = Self::NAME,
            search_len = args.search_block.len(),
            replace_len = args.replace_block.len()
        );
        span.record_model_input(&serde_json::json!({
            "search_preview": truncate_for_telemetry(&args.search_block, 180),
            "replace_preview": truncate_for_telemetry(&args.replace_block, 180),
            "search_len": args.search_block.len(),
            "replace_len": args.replace_block.len(),
        }));
        let _guard = span.enter();

        println!(
            "   ✏️ Tool Call: CodeSnippetReplace search='{}' replace='{}'",
            args.search_block.lines().next().unwrap_or("..."),
            args.replace_block.lines().next().unwrap_or("...")
        );

        // Find the latest generated script
        let script_path = find_latest_script(&self.output_dir).ok_or_else(|| {
            CodeReplaceError("No generated script found in output directory.".into())
        })?;

        let code = std::fs::read_to_string(&script_path)
            .map_err(|e| CodeReplaceError(format!("Failed to read script: {}", e)))?;

        // Apply search and replace using the infra module's function
        let new_code =
            crate::infra::apply_search_and_replace(&code, &args.search_block, &args.replace_block)
                .map_err(|e| CodeReplaceError(format!("Replace failed: {}", e)))?;

        // Write back the modified code
        std::fs::write(&script_path, &new_code)
            .map_err(|e| CodeReplaceError(format!("Failed to write updated code: {}", e)))?;

        // Count lines changed
        let old_line_count = code.lines().count();
        let new_line_count = new_code.lines().count();
        let line_diff = new_line_count as i64 - old_line_count as i64;

        let output = format!(
            "✅ Code updated in {}. Lines: {} → {} ({}{}).\n\
             Replaced:\n```\n{}\n```\nWith:\n```\n{}\n```",
            script_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy(),
            old_line_count,
            new_line_count,
            if line_diff >= 0 { "+" } else { "" },
            line_diff,
            args.search_block,
            args.replace_block
        );
        span.record_model_output(&serde_json::json!({
            "status": "ok",
            "line_diff": line_diff,
            "result_len": output.len(),
        }));
        Ok(output)
    }
}

// ---------------------------------------------------------------------------
// Helper: find the latest generated script
// ---------------------------------------------------------------------------

/// Finds the most recently modified `.py` script in any output subdirectory.
fn find_latest_script(output_dir: &std::path::Path) -> Option<PathBuf> {
    let mut latest: Option<(PathBuf, std::time::SystemTime)> = None;

    // Walk output subdirectories to find generated_script_*.py
    if let Ok(entries) = std::fs::read_dir(output_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir()
                && let Ok(sub_entries) = std::fs::read_dir(entry.path())
            {
                for sub_entry in sub_entries.flatten() {
                    let path = sub_entry.path();
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    if name.starts_with("generated_script_")
                        && name.ends_with(".py")
                        && let Ok(metadata) = path.metadata()
                        && let Ok(modified) = metadata.modified()
                        && latest.as_ref().is_none_or(|(_, t)| modified > *t)
                    {
                        latest = Some((path, modified));
                    }
                }
            }
        }
    }

    latest.map(|(path, _)| path)
}

fn truncate_for_telemetry(input: &str, max_chars: usize) -> String {
    let mut preview: String = input.chars().take(max_chars).collect();
    if input.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_latest_returns_none_for_empty_dir() {
        let result = find_latest_script(std::path::Path::new("/tmp/nonexistent_test_dir_12345"));
        assert!(result.is_none());
    }

    #[test]
    fn test_code_search_default() {
        let search = CodeSnippetSearch::new();
        assert_eq!(search.output_dir, PathBuf::from(".output"));
    }

    #[test]
    fn test_code_replace_default() {
        let replace = CodeSnippetReplace::new();
        assert_eq!(replace.output_dir, PathBuf::from(".output"));
    }
}
