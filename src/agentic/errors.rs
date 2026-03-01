//! Error taxonomy for the active multi-agent orchestration system.
//!
//! Use these variants to classify failures by subsystem (context, tooling, provider,
//! orchestration). This keeps high-level handling logic explicit and testable.

use thiserror::Error;

/// Canonical error enum for agentic runtime operations.
///
/// # Examples
///
/// ```
/// use build123d_cad::agentic::AgentError;
///
/// let err = AgentError::ToolError("tool timeout".to_string());
/// assert!(err.to_string().contains("Tool Error"));
/// ```
#[derive(Error, Debug)]
pub enum AgentError {
    /// Context mismatch or serialization failure.
    #[error("Context Error: {0}")]
    ContextError(String),

    /// Errors occurring during tool execution.
    #[error("Tool Error: {0}")]
    ToolError(String),

    /// Errors in the orchestration loop.
    #[error("Orchestrator Error: {0}")]
    OrchestrationError(String),

    /// Failed to parse a delegation logic string.
    #[error("Delegation Error: {0}")]
    DelegationParseError(String),

    /// Lower-level AI Provider error (e.g. API failure).
    #[error("AI Provider Error: {0}")]
    ProviderError(#[from] rig::completion::PromptError),

    /// Environment configuration missing or invalid.
    #[error("Environment Error: {0}")]
    EnvError(String),

    /// File system operation failed.
    #[error("File System Error: {0}")]
    FileSystemError(String),

    /// Orchestrator step limit reached.
    #[error("Max steps reached ({0}) without resolution")]
    MaxStepsReached(u32),

    /// Catch-all for other errors.
    #[error("Unknown Agent Error: {0}")]
    Unknown(#[from] anyhow::Error),
}
