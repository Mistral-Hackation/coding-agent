// use anyhow::anyhow;
use rig::completion::ToolDefinition;
use rig::telemetry::SpanCombinator;
use rig::tool::Tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serpscraper::get_markdown_for_query;
use std::time::Duration;

/// Argument for a web search query.
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct WebQuery {
    /// The search string to send to the search engine.
    pub query: String,
}

/// Error type for WebSearcher operations.
#[derive(Debug)]
pub struct WebSearchError(String);

impl std::fmt::Display for WebSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WebSearchError {}

/// A tool for performing web searches to gather external information.
///
/// Uses `serpscraper` which internally launches a headless Chromium browser.
/// Chromium's GPU subprocess produces harmless stderr noise (SharedImageManager
/// warnings) which we suppress by temporarily redirecting file descriptor 2.
#[derive(Serialize, Deserialize)]
pub struct WebSearcher;

impl Tool for WebSearcher {
    const NAME: &'static str = "WebSearcher";
    type Error = WebSearchError;
    type Args = WebQuery;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(WebQuery);
        let parameters = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
        ToolDefinition {
            name: "WebSearcher".to_string(),
            description: "A research tool. Use this to find best practices, domain-specific knowledge, or to verify assumptions about the user's goal.".to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let span = tracing::info_span!(
            "tool.web_search",
            tool_name = Self::NAME,
            query_len = args.query.len()
        );
        span.record_model_input(&serde_json::json!({
            "query_preview": truncate_for_telemetry(&args.query, 180),
            "query_len": args.query.len(),
        }));
        let _guard = span.enter();

        println!("   🛠️ Tool Call: WebSearcher args='{}'", args.query);
        let api_key = std::env::var("SERPER_API_KEY")
            .map_err(|e| WebSearchError(format!("SERPER_API_KEY not set: {}", e)))?;
        let timeout_secs = web_search_timeout_seconds();
        let timeout = Duration::from_secs(timeout_secs);
        let started_at = std::time::Instant::now();

        // Suppress Chromium GPU noise from stderr during the scraper call.
        // serpscraper spawns a headless Chromium browser whose GPU subprocess
        // writes harmless warnings (SharedImageManager, DEPRECATED_ENDPOINT)
        // to our inherited stderr, polluting the console output.
        let markdown = tokio::time::timeout(
            timeout,
            suppress_stderr(async { get_markdown_for_query(&args.query, &api_key).await }),
        )
        .await
        .map_err(|_| {
            WebSearchError(format!(
                "Web search timed out after {}s for query: {}",
                timeout_secs,
                truncate_for_telemetry(&args.query, 120)
            ))
        })?
        .map_err(|e| WebSearchError(format!("SerpScraper error: {}", e)))?;

        let duration_ms = started_at.elapsed().as_millis();

        if markdown.len() > 2000 {
            let truncated: String = markdown.chars().take(2000).collect();
            span.record_model_output(&serde_json::json!({
                "result_len": markdown.len(),
                "truncated": true,
                "duration_ms": duration_ms,
            }));
            Ok(format!("{}... (truncated)", truncated))
        } else {
            span.record_model_output(&serde_json::json!({
                "result_len": markdown.len(),
                "truncated": false,
                "duration_ms": duration_ms,
            }));
            Ok(markdown)
        }
    }
}

/// Temporarily redirects stderr to `/dev/null` while running a future,
/// then restores it. This suppresses noise from child processes (e.g.
/// Chromium GPU warnings) without affecting our own error logging.
///
/// On failure to redirect (e.g. non-Unix), the future runs normally
/// with stderr intact — a graceful degradation.
async fn suppress_stderr<F, T, E>(future: F) -> Result<T, E>
where
    F: std::future::Future<Output = Result<T, E>>,
{
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;

        // Save the current stderr file descriptor
        let original_stderr = unsafe { libc::dup(2) };
        if original_stderr < 0 {
            // dup() failed — just run without suppression
            return future.await;
        }

        // Open /dev/null and redirect stderr to it
        if let Ok(dev_null) = std::fs::File::open("/dev/null") {
            let dev_null_fd = dev_null.as_raw_fd();
            unsafe { libc::dup2(dev_null_fd, 2) };
        }

        let result = future.await;

        // Restore original stderr
        unsafe {
            libc::dup2(original_stderr, 2);
            libc::close(original_stderr);
        }

        result
    }

    #[cfg(not(unix))]
    {
        future.await
    }
}

fn truncate_for_telemetry(input: &str, max_chars: usize) -> String {
    let mut preview: String = input.chars().take(max_chars).collect();
    if input.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

fn web_search_timeout_seconds() -> u64 {
    std::env::var("WEB_SEARCH_TIMEOUT_SECS")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(25)
}
