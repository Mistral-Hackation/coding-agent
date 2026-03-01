//! Error types for staged generation workflows.
//!
//! The active multi-agent runtime uses [`crate::agentic::AgentError`], while this
//! module supports shared generation diagnostics used by stateful workflow transitions.

use thiserror::Error;

/// Canonical error enum for generation and transition diagnostics.
///
/// # Examples
///
/// ```
/// use build123d_cad::errors::GenError;
///
/// let err = GenError::AgentFailure("provider timeout".to_string());
/// assert!(err.to_string().contains("Agent Failure"));
/// ```
#[derive(Debug, Error)]
pub enum GenError {
    /// **AI Agent Failure**
    #[error("🤖 Agent Failure: {0}")]
    AgentFailure(String),

    /// **Filesystem Error**
    #[error("💾 IO Error: {0}")]
    Io(#[from] std::io::Error),

    /// **Concurrency Error**
    #[error("🔒 Mutex Poisoned: {0}")]
    PoisonedLock(String),

    /// **Python Syntax Error**
    #[error("📝 Python Syntax Error: {message} (Line: {line_number:?})")]
    SyntaxError {
        /// The error message returned by the Python parser.
        message: String,
        /// The line number where the syntax error occurred.
        line_number: Option<usize>,
    },
}

/// Converts poisoned lock errors into [`GenError::PoisonedLock`].
///
/// # Examples
///
/// ```ignore
/// use build123d_cad::errors::GenError;
///
/// fn as_gen_error<T>(error: std::sync::PoisonError<T>) -> GenError {
///     error.into()
/// }
/// ```
impl<T> From<std::sync::PoisonError<T>> for GenError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        GenError::PoisonedLock(e.to_string())
    }
}
