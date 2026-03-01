//! ExampleSearcher tool — agents search the local build123d example corpus
//! via command-line `grep`.
//!
//! The corpus lives at a configurable directory (default:
//! `/Users/hamzeghalebi/Downloads/python_files_merged_latest/`) and contains
//! hundreds of real build123d Python model functions, each annotated with a
//! `# Description:` comment and a function body showing idiomatic build123d
//! patterns.
//!
//! Agents call this tool with a natural-language query (e.g.
//! `"fillet cylinder bolt holes"`) and receive matching code snippets
//! complete with their descriptions.

use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The default location of the build123d example corpus.
const DEFAULT_EXAMPLES_PATH: &str = "/Users/hamzeghalebi/Downloads/python_files_merged_latest";

/// Maximum characters returned to the agent per search.
const MAX_OUTPUT_CHARS: usize = 6000;

// ---------------------------------------------------------------------------
// Tool args
// ---------------------------------------------------------------------------

/// Arguments for searching the build123d example corpus.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct ExampleQuery {
    /// Natural-language search keywords.
    /// Examples: "fillet cylinder", "bolt hole pattern",
    ///           "revolve profile", "CenterArc extrude"
    pub query: String,

    /// Maximum number of matching snippets to return (default: 3).
    pub max_results: Option<u8>,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Error type for ExampleSearcher operations.
#[derive(Debug)]
pub struct ExampleSearcherError(String);

impl std::fmt::Display for ExampleSearcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExampleSearcherError {}

// ---------------------------------------------------------------------------
// Tool struct
// ---------------------------------------------------------------------------

/// Searches a local directory of build123d Python example files using
/// command-line `grep`.
///
/// Each match returns the model description and the function body so the
/// agent can learn idiomatic build123d patterns from real code.
#[derive(Serialize, Deserialize, Clone)]
pub struct ExampleSearcher {
    /// Root directory containing `.py` example files.
    examples_path: PathBuf,
}

impl ExampleSearcher {
    /// Create with the default corpus path.
    pub fn new() -> Self {
        Self {
            examples_path: PathBuf::from(DEFAULT_EXAMPLES_PATH),
        }
    }

    /// Create with a custom path (for testing or alternative corpora).
    #[allow(dead_code)]
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            examples_path: path,
        }
    }
}

impl Default for ExampleSearcher {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// rig::tool::Tool implementation
// ---------------------------------------------------------------------------

impl Tool for ExampleSearcher {
    const NAME: &'static str = "ExampleSearcher";
    type Error = ExampleSearcherError;
    type Args = ExampleQuery;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(ExampleQuery);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);

        ToolDefinition {
            name: "ExampleSearcher".to_string(),
            description: "Search through hundreds of real build123d Python example files \
                 for relevant code patterns. Use keywords like 'fillet', 'bolt hole \
                 pattern', 'revolve', 'CenterArc', 'PolarLocations', 'chamfer', etc. \
                 Returns matching model functions with their descriptions. \
                 Prefer this over WebSearcher for build123d code patterns."
                .to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.example_search",
            tool_name = Self::NAME,
            query_len = args.query.len()
        );
        span.record_model_input(&serde_json::json!({
            "query_preview": truncate_for_telemetry(&args.query, 180),
            "query_len": args.query.len(),
            "max_results": args.max_results.unwrap_or(3),
        }));
        let _guard = span.enter();

        let max_results = args.max_results.unwrap_or(3) as usize;

        println!(
            "   🔍 Tool Call: ExampleSearcher query='{}' max_results={}",
            args.query, max_results
        );

        // Split query into individual keywords for grep -e chaining.
        let keywords: Vec<&str> = args
            .query
            .split_whitespace()
            .filter(|w| w.len() >= 2) // Skip one-letter words
            .collect();

        if keywords.is_empty() {
            span.record_model_output(&serde_json::json!({
                "status": "empty_keywords",
            }));
            return Ok("No valid search keywords provided.".to_string());
        }

        // Strategy: grep for the first keyword, then filter results containing
        // additional keywords in post-processing. We search description
        // comments (broadest match).
        let primary_keyword = keywords[0];

        let output = tokio::process::Command::new("grep")
            .args([
                "-r",
                "-i",
                "-n",
                "--include=*.py",
                "-B",
                "1", // 1 line before (captures `# Description:`)
                "-A",
                "40", // 40 lines after (captures full function body)
                primary_keyword,
            ])
            .arg(&self.examples_path)
            .output()
            .await
            .map_err(|e| ExampleSearcherError(format!("Failed to run grep: {}", e)))?;

        let raw = String::from_utf8_lossy(&output.stdout);

        if raw.is_empty() {
            span.record_model_output(&serde_json::json!({
                "status": "no_primary_matches",
            }));
            return Ok(format!(
                "No examples found matching '{}'. Try broader keywords.",
                args.query
            ));
        }

        // Split by grep's group separator and filter for matches that
        // contain ALL keywords (logical AND).
        let groups: Vec<&str> = raw.split("--\n").collect();

        let mut matched: Vec<String> = Vec::new();
        for group in &groups {
            let group_lower = group.to_lowercase();
            let is_all_match = keywords
                .iter()
                .all(|kw| group_lower.contains(&kw.to_lowercase()));

            if is_all_match {
                // Trim file path prefixes for readability
                let cleaned = group
                    .lines()
                    .map(|line| {
                        // Remove the long path prefix, keep relative path
                        if let Some(idx) = line.find("python_files_merged_latest/") {
                            &line[idx + "python_files_merged_latest/".len()..]
                        } else {
                            line
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                matched.push(cleaned);
            }

            if matched.len() >= max_results {
                break;
            }
        }

        if matched.is_empty() {
            span.record_model_output(&serde_json::json!({
                "status": "no_full_keyword_matches",
            }));
            return Ok(format!(
                "Found matches for '{}' but none matched ALL keywords. \
                 Try fewer/broader keywords.",
                args.query
            ));
        }

        let header = format!(
            "Found {} example(s) matching '{}' \
             (showing up to {}):\n\n",
            matched.len(),
            args.query,
            max_results
        );

        let body = matched.join("\n\n---\n\n");

        let result = format!("{}{}", header, body);

        // Truncate if too long
        if result.len() > MAX_OUTPUT_CHARS {
            let truncated: String = result.chars().take(MAX_OUTPUT_CHARS).collect();
            span.record_model_output(&serde_json::json!({
                "status": "ok",
                "matches": matched.len(),
                "result_len": result.len(),
                "truncated": true,
            }));
            Ok(format!("{}... (truncated)", truncated))
        } else {
            span.record_model_output(&serde_json::json!({
                "status": "ok",
                "matches": matched.len(),
                "result_len": result.len(),
                "truncated": false,
            }));
            Ok(result)
        }
    }
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
    fn test_default_path() {
        let searcher = ExampleSearcher::new();
        assert!(
            searcher
                .examples_path
                .ends_with("python_files_merged_latest")
        );
    }

    #[test]
    fn test_custom_path() {
        let searcher = ExampleSearcher::with_path(PathBuf::from("/tmp/examples"));
        assert_eq!(searcher.examples_path, PathBuf::from("/tmp/examples"));
    }
}
