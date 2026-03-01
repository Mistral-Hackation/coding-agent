//! Infrastructure adapters used by the active runtime.
//!
//! This module hosts side-effecting integrations:
//!
//! - [`crate::infra::GitJournal`] for traceable checkpoints.
//! - [`crate::infra::FileLogger`] for local observability.
//! - [`crate::infra::PythonLinter`] for fast syntax feedback on generated scripts.
//! - [`crate::infra::Fabricator`] as a dependency bundle consumed by runtime orchestration.
//! - [`crate::infra::apply_search_and_replace`] for deterministic patch application.
//!
//! Design intent: keep orchestration logic in higher layers and isolate process/file
//! interactions in reusable helpers.

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::sync::{Arc, Mutex};

// --- TOOL 1: The Historian (Git) ---

/// Lightweight wrapper around the system `git` CLI.
///
/// The orchestrator uses this adapter to create an append-only audit trail of major
/// events. This is intentionally simple and best-effort: failures are ignored so git
/// problems do not stop generation loops.
pub struct GitJournal {
    root: String,
}

impl GitJournal {
    /// Creates a new journal rooted at `root`.
    ///
    /// The constructor eagerly runs `git init` in the target directory. Any failure is
    /// intentionally ignored, because journaling should not block the main execution path.
    ///
    /// # Arguments
    ///
    /// * `root` - The directory path where the git repository should be initialized.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use build123d_cad::infra::GitJournal;
    ///
    /// let journal = GitJournal::new(".");
    /// journal.snapshot("bootstrap", "initialized repo");
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn new(root: &str) -> Self {
        let _ = Command::new("git").current_dir(root).arg("init").output();
        Self { root: root.into() }
    }

    /// Records a snapshot commit with actor attribution.
    ///
    /// Internally runs:
    ///
    /// 1. `git add .`
    /// 2. `git commit --allow-empty -m "[actor] msg"`
    ///
    /// `--allow-empty` is important for preserving reasoning checkpoints even when no
    /// file content changed.
    ///
    /// # Arguments
    ///
    /// * `actor` - The name of the component performing the action (e.g., "Architect").
    /// * `msg` - A brief description of the action.
    /// # Examples
    ///
    /// ```no_run
    /// use build123d_cad::infra::GitJournal;
    ///
    /// let journal = GitJournal::new(".");
    /// journal.snapshot("coder", "generated initial draft");
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn snapshot(&self, actor: &str, msg: &str) {
        let _ = Command::new("git")
            .current_dir(&self.root)
            .args(["add", "."])
            .output();
        let log = format!("[{}] {}", actor, msg);
        let _ = Command::new("git")
            .current_dir(&self.root)
            .args(["commit", "--allow-empty", "-m", &log])
            .output();
    }
}

// --- TOOL 2: The Scribe (File Logger) ---

/// Minimal append-only logger that mirrors output to stdout and disk.
///
/// [`Fabricator`] typically wraps this in `Arc<Mutex<_>>` so multiple runtime
/// components can emit ordered logs.
pub struct FileLogger {
    file: std::fs::File,
}

impl FileLogger {
    /// Opens (or creates) a log file in append mode.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use build123d_cad::infra::FileLogger;
    ///
    /// let mut logger = FileLogger::new("run.log")?;
    /// logger.log("starting run");
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns any filesystem error emitted by [`OpenOptions::open`].
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file })
    }

    /// Writes a timestamped entry to stdout and the backing file.
    ///
    /// File write failures are ignored to keep logging best-effort.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use build123d_cad::infra::FileLogger;
    ///
    /// let mut logger = FileLogger::new("run.log")?;
    /// logger.log("mission started");
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    pub fn log(&mut self, msg: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let output = format!(
            "[{}] {}
",
            timestamp, msg
        );
        print!("{}", output);
        let _ = self.file.write_all(output.as_bytes());
    }
}

// --- CONTEXT: The Fabricator ---

/// Shared dependency bundle for generation and repair flows.
///
/// This type centralizes external dependencies (LLM, git, logging, linting, knowledge tools)
/// so call sites can focus on state transitions rather than setup ceremony.
pub struct Fabricator {
    /// The AI Brain (Anthropic Claude via Azure MaaS)
    pub agent: crate::types::ClaudeAgent,
    /// Shared access to Git (Thread-safe)
    pub git: Arc<Mutex<GitJournal>>,
    /// Shared access to Logs (Thread-safe)
    pub logger: Arc<Mutex<FileLogger>>,
    /// The Python syntax checker
    pub linter: PythonLinter,
    /// Where files should be saved
    pub output_dir: std::path::PathBuf,
    /// Knowledge base for reading domain expertise
    pub knowledge_base: crate::agentic::tools::knowledge_base::KnowledgeBase,
    /// Tool for saving learned insights
    pub learn_knowledge: crate::agentic::tools::knowledge_base::LearnKnowledge,
}

impl Fabricator {
    /// Builds a [`Fabricator`] from explicitly provisioned runtime dependencies.
    ///
    /// The constructor wires default knowledge tools for the `"coder"` agent role.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use build123d_cad::infra::{Fabricator, FileLogger, GitJournal};
    /// use std::path::PathBuf;
    /// use std::sync::{Arc, Mutex};
    ///
    /// # let agent: build123d_cad::types::ClaudeAgent = todo!();
    /// let fabricator = Fabricator::new(
    ///     agent,
    ///     Arc::new(Mutex::new(GitJournal::new("."))),
    ///     Arc::new(Mutex::new(FileLogger::new("run.log").expect("logger"))),
    ///     PathBuf::from("output"),
    /// );
    /// # let _ = fabricator;
    /// ```
    pub fn new(
        agent: crate::types::ClaudeAgent,
        git: Arc<Mutex<GitJournal>>,
        logger: Arc<Mutex<FileLogger>>,
        output_dir: std::path::PathBuf,
    ) -> Self {
        Self {
            agent,
            git,
            logger,
            linter: PythonLinter,
            output_dir,
            // Default knowledge tools for "coder" agent
            knowledge_base: crate::agentic::tools::knowledge_base::KnowledgeBase::new("coder"),
            learn_knowledge: crate::agentic::tools::knowledge_base::LearnKnowledge::new("coder"),
        }
    }

    /// Convenience wrapper around the internal logger mutex.
    ///
    /// If the logger mutex is poisoned, the message is dropped silently to preserve
    /// forward progress in long-running missions.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # let fabricator: build123d_cad::infra::Fabricator = todo!();
    /// fabricator.log("reviewer approved the draft");
    /// ```
    pub fn log(&self, msg: &str) {
        if let Ok(mut logger) = self.logger.lock() {
            logger.log(msg);
        }
    }

    /// Queries the knowledge base tool for contextual guidance.
    ///
    /// Returns an empty string when the tool errors or no relevant entry exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn demo(fabricator: &build123d_cad::infra::Fabricator) {
    /// let hints = fabricator.query_knowledge("flange bolt-circle pattern").await;
    /// # let _ = hints;
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This API intentionally does not surface errors. Tool failures degrade to an
    /// empty response.
    pub async fn query_knowledge(&self, topic: &str) -> String {
        use rig::tool::Tool;
        let query = crate::agentic::tools::knowledge_base::KnowledgeQuery {
            topic: topic.to_string(),
        };
        self.knowledge_base.call(query).await.unwrap_or_default()
    }

    /// Persists a learned insight through the learning tool.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # async fn demo(fabricator: &build123d_cad::infra::Fabricator) {
    /// let ok = fabricator
    ///     .save_learning(
    ///         "Fix Pattern",
    ///         "Use `Part()` context before exporting.",
    ///         "review loop",
    ///     )
    ///     .await;
    /// assert!(ok || !ok);
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This API intentionally returns a `bool` instead of bubbling tool errors.
    pub async fn save_learning(&self, category: &str, content: &str, source: &str) -> bool {
        use rig::tool::Tool;
        let input = crate::agentic::tools::knowledge_base::LearnInput {
            category: category.to_string(),
            content: content.to_string(),
            source: source.to_string(),
        };
        self.learn_knowledge.call(input).await.is_ok()
    }
}

// --- TOOL 3: The Scalpel (Search & Replace) ---

/// Applies a deterministic search-and-replace edit over `full_code`.
///
/// This helper is intentionally strict: it only edits when `search_block` is found
/// exactly. That behavior is useful in repair workflows where fuzzy edits can mask
/// drift between model intent and actual source state.
///
/// # Examples
///
/// ```
/// use build123d_cad::infra::apply_search_and_replace;
///
/// let code = "def foo():\n    return 1";
/// let search = "return 1";
/// let replace = "return 2";
///
/// let new_code = apply_search_and_replace(code, search, replace).unwrap();
/// assert_eq!(new_code, "def foo():\n    return 2");
/// ```
///
/// # Errors
///
/// Returns `Err(String)` when `search_block` does not exist in `full_code`.
///
/// # Panics
///
/// This function does not intentionally panic.
pub fn apply_search_and_replace(
    full_code: &str,
    search_block: &str,
    replace_block: &str,
) -> Result<String, String> {
    if let Some(start_idx) = full_code.find(search_block) {
        let mut new_code = String::with_capacity(full_code.len() + replace_block.len());
        new_code.push_str(&full_code[..start_idx]);
        new_code.push_str(replace_block);
        new_code.push_str(&full_code[start_idx + search_block.len()..]);
        return Ok(new_code);
    }
    Err("Search block not found in source code. Ensure exact match.".to_string())
}

// --- TOOL 4: The Linter ---

/// Python syntax validator used as a pre-flight check for generated code.
///
/// The validator delegates to `python3` and attempts to compile the input source from
/// stdin. It does not execute script side effects.
pub struct PythonLinter;

impl PythonLinter {
    /// Validates Python syntax by compiling source with `python3`.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::infra::PythonLinter;
    ///
    /// let linter = PythonLinter;
    /// assert!(linter.validate("x = 1").is_ok());
    /// assert!(linter.validate("def broken(").is_err());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` when:
    ///
    /// - `python3` cannot be spawned, or
    /// - Python reports a syntax error.
    pub fn validate(&self, code: &str) -> Result<(), String> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new("python3")
            .arg("-c")
            .arg("import sys; compile(sys.stdin.read(), '<string>', 'exec')")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to spawn python linter: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(code.as_bytes());
        }

        let output = child.wait_with_output().map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(error.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_and_replace_exact() {
        let code = "A\nB\nC";
        let result = apply_search_and_replace(code, "B", "X").unwrap();
        assert_eq!(result, "A\nX\nC");
    }
}
