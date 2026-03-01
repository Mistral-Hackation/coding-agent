#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! # build123d_cad
//!
//! `build123d_cad` is a Rust orchestration layer for generating parametric
//! [build123d](https://build123d.readthedocs.io/) Python scripts with LLM agents.
//!
//! The crate exposes one active architectural track:
//!
//! 1. [`agentic`]: a multi-agent supervisor loop with specialist reviewers.
//!
//! ## Architecture At A Glance
//!
//! The active flow is orchestrated by [`agentic::Orchestrator`], which routes control
//! between implementations of [`agentic::Specialist`]:
//!
//! ```text
//! supervisor -> researcher -> coder -> reviewers -> supervisor
//! ```
//!
//! Each turn mutates [`agentic::GlobalContext`] and returns a [`agentic::TurnResult`]
//! that determines the next transition.
//!
//! ## Module Guide
//!
//! - [`agentic`]: active orchestration engine, agents, tools, and agent errors.
//! - [`infra`]: shared runtime adapters (`GitJournal`, logging, linting, knowledge tools).
//! - [`types`]: domain state types such as [`types::Artifact`] and [`types::Blueprint`].
//! - [`errors`]: workflow diagnostic error type (`GenError`) used by state transitions.
//! - [`setup`]: environment checks for Python/build123d availability.
//!
//! ## Quick Start
//!
//! Even without connecting to an LLM provider, you can use the domain types to model a
//! CAD request:
//!
//! ```
//! use build123d_cad::types::{Artifact, Blueprint};
//!
//! let request = Artifact {
//!     name: "mounting_bracket".to_string(),
//!     state: Blueprint {
//!         prompt: "A parametric L bracket with two bolt holes".to_string(),
//!     },
//! };
//!
//! assert_eq!(request.name, "mounting_bracket");
//! ```
//!
//! For a full mission loop, construct an [`agentic::Orchestrator`] with concrete
//! specialists from [`agentic::agents`].
//!
//! # Examples
//!
//! ```rust,no_run
//! use build123d_cad::agentic::{GlobalContext, ReviewConsensus};
//!
//! let mut ctx = GlobalContext::new("Design a gearbox housing".to_string());
//! ctx.review_consensus = ReviewConsensus::new();
//! assert_eq!(ctx.step_count, 0);
//! ```
//!
//! # Panics
//!
//! This crate does not intentionally panic in public APIs. Runtime failures are represented
//! through `Result` types where available (for example, [`agentic::AgentError`]).
//!
//! # Errors
//!
//! Error surfaces are split by subsystem:
//!
//! - [`agentic::AgentError`] for the active multi-agent workflow.
//! - [`errors::GenError`] for domain state diagnostics used across transition handling.
//!
//! # Safety
//!
//! This crate does not expose `unsafe` public APIs.

/// Active multi-agent orchestration system.
pub mod agentic;
/// Workflow diagnostics for generation and transition failures.
pub mod errors;
/// Shared runtime infrastructure (git, logging, linting, knowledge adapters).
pub mod infra;
/// Environment validation helpers for Python/build123d.
pub mod setup;
/// OpenTelemetry bootstrap and lifecycle helpers.
pub mod telemetry;
/// Shared domain and state types.
pub mod types;

/// Interactive 3D viewer for generated STL files.
pub mod viewer;

/// Re-export of the shared execution context from [`infra`].
pub use infra::Fabricator;
/// Re-export of the generic domain container from [`types`].
pub use types::Artifact;
