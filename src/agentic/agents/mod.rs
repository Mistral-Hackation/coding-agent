//! Specialized Agents Module
//!
//! This module defines the concrete implementations of the [`crate::agentic::Specialist`] trait.
//! Each agent is a distinct "Persona" with a specific role, toolset, and prompt.

/// The Coder agent implementation.
pub mod coder;
/// The Compliance Reviewer agent implementation.
pub mod compliance_reviewer;
/// The Intent Reviewer agent implementation.
pub mod intent_reviewer;
/// The Oil & Gas Reviewer agent implementation.
pub mod oil_gas_reviewer;
/// The Physics Reviewer agent implementation.
pub mod physics_reviewer;
/// Centralized agent prompts/preambles.
pub mod prompts;
/// The Researcher agent implementation.
pub mod researcher;
/// The Code Reviewer agent implementation.
pub mod reviewer;
/// The Supervisor agent implementation.
pub mod supervisor;
/// The Supply Reviewer agent implementation.
pub mod supply_reviewer;
/// Shared utility functions for agents.
pub mod utils;

pub use coder::Coder;
pub use compliance_reviewer::ComplianceReviewer;
pub use intent_reviewer::IntentReviewer;
pub use oil_gas_reviewer::OilGasReviewer;
pub use physics_reviewer::PhysicsReviewer;
pub use researcher::Researcher;
pub use reviewer::Reviewer;
pub use supervisor::Supervisor;
pub use supply_reviewer::SupplyReviewer;
