//! Build123d documentation tool backed by a local ReadTheDocs export.

use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_MAX_LEN: usize = 4_000;

/// Argument for local build123d documentation lookups.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct Build123dDocsQuery {
    /// Search topic, keyword, or phrase for documentation content.
    pub topic: String,
}

/// Error type for Build123d docs reads.
#[derive(Debug)]
pub struct Build123dDocsError(String);

impl std::fmt::Display for Build123dDocsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Build123dDocsError {}

/// A tool for reading build123d documentation from the repository-local
/// `build123d_readthedocs.txt` file.
#[derive(Serialize, Deserialize, Clone)]
pub struct Build123dDocsReader {
    /// Optional override for the documentation path.
    #[serde(skip)]
    docs_path: PathBuf,
}

impl Build123dDocsReader {
    /// Create a new documentation reader using the default file path.
    pub fn new() -> Self {
        Self {
            docs_path: default_docs_path(),
        }
    }

    /// Create a reader with a custom documentation path (mostly useful for tests).
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        Self {
            docs_path: path.as_ref().to_path_buf(),
        }
    }
}

impl Default for Build123dDocsReader {
    fn default() -> Self {
        Self::new()
    }
}

fn search_sections(content: &str, topic: &str) -> String {
    let topic_lower = topic.to_lowercase();
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();
    let mut current_section: Option<String> = None;
    let mut section_content = Vec::new();
    let mut section_matches = false;

    for line in lines {
        if line.starts_with("# ") || line.starts_with("## ") || line.starts_with("### ") {
            if section_matches && let Some(ref section) = current_section {
                results.push(format!("{}\n{}", section, section_content.join("\n")));
            }
            current_section = Some(line.to_string());
            section_content.clear();
            section_matches = line.to_lowercase().contains(&topic_lower);
            continue;
        }

        if line.to_lowercase().contains(&topic_lower) {
            section_matches = true;
        }
        section_content.push(line.to_string());
    }

    if section_matches && let Some(ref section) = current_section {
        results.push(format!("{}\n{}", section, section_content.join("\n")));
    }

    if results.is_empty() {
        let truncated: String = content.chars().take(DEFAULT_MAX_LEN).collect();
        format!(
            "No specific match for '{}'. Here's the full documentation (truncated):\n\n{}",
            topic, truncated
        )
    } else {
        results.join("\n\n---\n\n")
    }
}

impl Tool for Build123dDocsReader {
    const NAME: &'static str = "Build123dDocs";
    type Error = Build123dDocsError;
    type Args = Build123dDocsQuery;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(Build123dDocsQuery);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "Build123dDocs".to_string(),
            description:
                "Read project-local build123d documentation from build123d_readthedocs.txt. \
                Use this tool to look up API signatures, CAD conventions, and reference examples."
                    .to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.build123d_docs",
            tool_name = Self::NAME,
            topic_len = args.topic.len()
        );
        span.record_model_input(&serde_json::json!({
            "topic": truncate_for_telemetry(&args.topic, 180),
            "docs_path": self.docs_path.display().to_string(),
        }));
        let _guard = span.enter();

        println!("   📘 Tool Call: Build123dDocs topic='{}'", args.topic);

        let content = fs::read_to_string(&self.docs_path).map_err(|e| {
            Build123dDocsError(format!(
                "Failed to read docs at {:?}: {}",
                self.docs_path, e
            ))
        })?;
        let result = search_sections(&content, &args.topic);

        if result.len() > DEFAULT_MAX_LEN {
            let truncated: String = result.chars().take(DEFAULT_MAX_LEN).collect();
            span.record_model_output(&serde_json::json!({
                "result_len": result.len(),
                "truncated": true,
                "docs_path": self.docs_path.display().to_string(),
            }));
            Ok(format!("{}... (truncated)", truncated))
        } else {
            span.record_model_output(&serde_json::json!({
                "result_len": result.len(),
                "truncated": false,
                "docs_path": self.docs_path.display().to_string(),
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

fn default_docs_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("knowledge")
        .join("build123d_readthedocs.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_docs_path() {
        let tool = Build123dDocsReader::with_path("docs/knowledge/build123d_readthedocs.txt");
        assert_eq!(
            tool.docs_path,
            PathBuf::from("docs/knowledge/build123d_readthedocs.txt")
        );
    }
}
