//! KnowledgeBase tool for agents to access their domain-specific knowledge files.

use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Argument for knowledge base queries.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct KnowledgeQuery {
    /// The topic or section to search for in the knowledge base.
    /// Use keywords like "thread specifications", "approval criteria", "checklist", etc.
    pub topic: String,
}

/// Error type for KnowledgeBase operations.
#[derive(Debug)]
pub struct KnowledgeBaseError(String);

impl std::fmt::Display for KnowledgeBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for KnowledgeBaseError {}

/// A tool for accessing agent-specific knowledge base files.
/// Each agent has a corresponding markdown file in docs/knowledge/ with domain expertise.
#[derive(Serialize, Deserialize, Clone)]
pub struct KnowledgeBase {
    /// The agent name (used to find the corresponding knowledge file).
    agent_name: String,
    /// Optional base path for knowledge files (defaults to docs/knowledge/).
    #[serde(skip)]
    base_path: Option<PathBuf>,
}

impl KnowledgeBase {
    /// Create a new KnowledgeBase tool for a specific agent.
    ///
    /// # Arguments
    /// * `agent_name` - The agent name (e.g., "coder", "oilgas_reviewer", "compliance_reviewer")
    pub fn new(agent_name: &str) -> Self {
        Self {
            agent_name: agent_name.to_string(),
            base_path: None,
        }
    }

    /// Create with a custom base path (for testing).
    pub fn with_base_path(agent_name: &str, base_path: PathBuf) -> Self {
        Self {
            agent_name: agent_name.to_string(),
            base_path: Some(base_path),
        }
    }

    /// Get the path to the knowledge file for this agent.
    fn knowledge_file_path(&self) -> PathBuf {
        let base = self
            .base_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("docs/knowledge"));

        // Convert agent name to knowledge file name
        // e.g., "oilgas_reviewer" -> "oilgas_knowledge.md"
        //       "physics_reviewer" -> "physics_knowledge.md"
        //       "coder" -> "coder_knowledge.md"
        let file_name = if self.agent_name.ends_with("_reviewer") {
            let prefix = self.agent_name.trim_end_matches("_reviewer");
            format!("{}_knowledge.md", prefix)
        } else {
            format!("{}_knowledge.md", self.agent_name)
        };

        base.join(file_name)
    }

    /// Search the knowledge content for sections matching the topic.
    fn search_content(&self, content: &str, topic: &str) -> String {
        let topic_lower = topic.to_lowercase();
        let lines: Vec<&str> = content.lines().collect();
        let mut results = Vec::new();
        let mut current_section: Option<String> = None;
        let mut section_content = Vec::new();
        let mut section_matches = false;

        for line in lines {
            // Detect section headers
            if line.starts_with("## ") || line.starts_with("### ") {
                // Save previous section if it matched
                if section_matches && let Some(ref section) = current_section {
                    results.push(format!("{}\n{}", section, section_content.join("\n")));
                }
                // Start new section
                current_section = Some(line.to_string());
                section_content.clear();
                section_matches = line.to_lowercase().contains(&topic_lower);
            } else {
                // Check if content contains topic
                if line.to_lowercase().contains(&topic_lower) {
                    section_matches = true;
                }
                section_content.push(line.to_string());
            }
        }

        // Check last section
        if section_matches && let Some(ref section) = current_section {
            results.push(format!("{}\n{}", section, section_content.join("\n")));
        }

        if results.is_empty() {
            // Return full content if no specific match (truncated)
            let truncated: String = content.chars().take(3000).collect();
            format!(
                "No specific match for '{}'. Here's the full knowledge base (truncated):\n\n{}",
                topic, truncated
            )
        } else {
            results.join("\n\n---\n\n")
        }
    }
}

impl Tool for KnowledgeBase {
    const NAME: &'static str = "KnowledgeBase";
    type Error = KnowledgeBaseError;
    type Args = KnowledgeQuery;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(KnowledgeQuery);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "KnowledgeBase".to_string(),
            description: format!(
                "Access the internal knowledge base for the {} agent. Contains domain-specific expertise, \
                 checklists, standards, and approval criteria. Use this BEFORE web search for known patterns. \
                 Query with keywords like 'thread specs', 'approval criteria', 'checklist', 'standards'.",
                self.agent_name
            ),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.knowledge_base",
            tool_name = Self::NAME,
            agent = %self.agent_name,
            topic_len = args.topic.len()
        );
        span.record_model_input(&serde_json::json!({
            "agent": self.agent_name,
            "topic_preview": truncate_for_telemetry(&args.topic, 180),
            "topic_len": args.topic.len(),
        }));
        let _guard = span.enter();

        let path = self.knowledge_file_path();
        println!(
            "   📚 Tool Call: KnowledgeBase[{}] topic='{}'",
            self.agent_name, args.topic
        );

        // Read the knowledge file
        let content = fs::read_to_string(&path).map_err(|e| {
            KnowledgeBaseError(format!(
                "Failed to read knowledge base at {:?}: {}",
                path, e
            ))
        })?;

        // Search for relevant sections
        let result = self.search_content(&content, &args.topic);

        // Truncate if too long
        if result.len() > 4000 {
            let truncated: String = result.chars().take(4000).collect();
            span.record_model_output(&serde_json::json!({
                "result_len": result.len(),
                "truncated": true,
            }));
            Ok(format!("{}... (truncated)", truncated))
        } else {
            span.record_model_output(&serde_json::json!({
                "result_len": result.len(),
                "truncated": false,
            }));
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_file_path() {
        let kb = KnowledgeBase::new("oilgas_reviewer");
        assert!(kb.knowledge_file_path().ends_with("oilgas_knowledge.md"));

        let kb = KnowledgeBase::new("coder");
        assert!(kb.knowledge_file_path().ends_with("coder_knowledge.md"));

        let kb = KnowledgeBase::new("physics_reviewer");
        assert!(kb.knowledge_file_path().ends_with("physics_knowledge.md"));
    }
}

// =============================================================================
// LEARN KNOWLEDGE TOOL - Allows agents to save new insights
// =============================================================================

/// Argument for learning/saving new knowledge.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct LearnInput {
    /// The category/section for this knowledge (e.g., "API Patterns", "Common Pitfalls", "New Standards")
    pub category: String,
    /// The new insight or knowledge to save
    pub content: String,
    /// Source of this knowledge (e.g., "WebSearch", "User Feedback", "Review Finding")
    pub source: String,
}

/// A tool for agents to save new insights to their knowledge base.
/// This enables continuous learning from web searches and feedback.
#[derive(Serialize, Deserialize, Clone)]
pub struct LearnKnowledge {
    /// The agent name (used to find the corresponding knowledge file).
    agent_name: String,
    /// Optional base path for knowledge files (defaults to docs/knowledge/).
    #[serde(skip)]
    base_path: Option<PathBuf>,
}

impl LearnKnowledge {
    /// Create a new LearnKnowledge tool for a specific agent.
    pub fn new(agent_name: &str) -> Self {
        Self {
            agent_name: agent_name.to_string(),
            base_path: None,
        }
    }

    /// Get the path to the knowledge file for this agent.
    #[allow(dead_code)]
    fn knowledge_file_path(&self) -> PathBuf {
        let base = self
            .base_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("docs/knowledge"));

        let file_name = if self.agent_name.ends_with("_reviewer") {
            let prefix = self.agent_name.trim_end_matches("_reviewer");
            format!("{}_knowledge.md", prefix)
        } else {
            format!("{}_knowledge.md", self.agent_name)
        };

        base.join(file_name)
    }

    /// Get the path to the learned knowledge file (separate from base knowledge).
    fn learned_file_path(&self) -> PathBuf {
        let base = self
            .base_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("docs/knowledge/learned"));

        let file_name = if self.agent_name.ends_with("_reviewer") {
            let prefix = self.agent_name.trim_end_matches("_reviewer");
            format!("{}_learned.md", prefix)
        } else {
            format!("{}_learned.md", self.agent_name)
        };

        base.join(file_name)
    }
}

impl Tool for LearnKnowledge {
    const NAME: &'static str = "LearnKnowledge";
    type Error = KnowledgeBaseError;
    type Args = LearnInput;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(LearnInput);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "LearnKnowledge".to_string(),
            description: format!(
                "IMPORTANT: Save insights to {} agent's memory! Call this tool whenever you: \
                 (1) Find useful info from WebSearcher, (2) Discover a pattern/best practice, \
                 (3) Learn something from reviewing code, (4) Complete a review (save what you learned). \
                 This builds your expertise over time. Don't hesitate - if it's useful, save it!",
                self.agent_name
            ),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.learn_knowledge",
            tool_name = Self::NAME,
            agent = %self.agent_name,
            category_len = args.category.len(),
            source = %args.source
        );
        span.record_model_input(&serde_json::json!({
            "agent": self.agent_name,
            "category": &args.category,
            "source": &args.source,
            "content_preview": truncate_for_telemetry(&args.content, 220),
            "content_len": args.content.len(),
        }));
        let _guard = span.enter();

        let path = self.learned_file_path();
        println!(
            "   🧠 Tool Call: LearnKnowledge[{}] category='{}' source='{}'",
            self.agent_name, args.category, args.source
        );

        // Ensure the learned directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                KnowledgeBaseError(format!("Failed to create learned directory: {}", e))
            })?;
        }

        // Read existing learned content or create new
        let existing = fs::read_to_string(&path).unwrap_or_default();

        // Format the new entry with timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M");
        let new_entry = format!(
            "\n## {} ({})\n\n*Source: {}*\n\n{}\n",
            args.category, timestamp, args.source, args.content
        );

        // Check if this content already exists (crude deduplication)
        if existing.contains(&args.content) {
            span.record_model_output(&serde_json::json!({
                "status": "skipped_duplicate",
                "path": path.display().to_string(),
            }));
            return Ok("Knowledge already exists in the learned file. Skipped.".to_string());
        }

        // Append to file
        let updated = if existing.is_empty() {
            format!(
                "# Learned Knowledge: {} Agent\n\n> Auto-generated insights from web searches and feedback.\n\n---\n{}",
                self.agent_name, new_entry
            )
        } else {
            format!("{}\n---\n{}", existing.trim(), new_entry)
        };

        fs::write(&path, updated)
            .map_err(|e| KnowledgeBaseError(format!("Failed to write learned knowledge: {}", e)))?;

        span.record_model_output(&serde_json::json!({
            "status": "stored",
            "path": path.display().to_string(),
        }));
        Ok(format!(
            "✅ Learned new insight in category '{}' from {}. Saved to {:?}",
            args.category, args.source, path
        ))
    }
}

fn truncate_for_telemetry(input: &str, max_chars: usize) -> String {
    let mut preview: String = input.chars().take(max_chars).collect();
    if input.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}
