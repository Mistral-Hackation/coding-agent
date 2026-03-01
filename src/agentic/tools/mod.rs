//! Tool modules used by agent specialists.
//!
//! These adapters expose filesystem, search, and knowledge capabilities through Rig's
//! typed tool interfaces.

/// System-facing tools (git snapshot, linting, code edits).
pub mod system;
/// Web search integration for external reference lookups.
pub mod web_searcher;

/// The Knowledge Base tool for agent-specific domain expertise.
pub mod knowledge_base;

/// Local Build123d documentation reader for `build123d_readthedocs.txt`.
pub mod build123d_docs;

/// The Example Searcher tool for finding patterns in the local build123d corpus.
pub mod example_searcher;

/// Code editing tools: search and replace snippets in generated Python scripts.
pub mod code_editor;
