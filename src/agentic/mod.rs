//! Core types and contracts for the active multi-agent runtime.
//!
//! This module defines the shared blackboard state
//! ([`crate::agentic::GlobalContext`]), the turn transition protocol
//! ([`crate::agentic::TurnResult`]), and the agent contract
//! ([`crate::agentic::Specialist`]).

/// Sub-module containing specific Agent implementations (Supervisor, Researcher, Coder).
pub mod agents;
/// Sub-module containing Agent Errors.
pub mod errors;
/// Sub-module containing the Orchestrator engine.
pub mod orchestrator;
/// Sub-module containing typed Tools (e.g. WebSearcher).
pub mod tools;

/// Re-export of all concrete agent types from [`agents`].
pub use agents::*;
/// Re-export of the agentic error type.
pub use errors::AgentError;
/// Re-export of orchestration engine types.
pub use orchestrator::*;
/// Re-export of tool types used by agents.
pub use tools::*;

use crate::types::Artifact;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Tracks reviewer approvals for the current draft.
///
/// This structure acts as a compact consensus bitmap, allowing the orchestrator to
/// determine whether review can terminate successfully.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewConsensus {
    /// Whether the Reviewer (code quality) has approved.
    pub reviewer: bool,
    /// Whether the PhysicsReviewer (geometry/physics) has approved.
    pub physics_reviewer: bool,
    /// Whether the IntentReviewer (user intent) has approved.
    pub intent_reviewer: bool,
    /// Whether the ComplianceReviewer (EU regulations) has approved.
    pub compliance_reviewer: bool,
}

impl ReviewConsensus {
    /// Creates an empty consensus state (all approvals set to `false`).
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let consensus = ReviewConsensus::new();
    /// assert_eq!(consensus.approval_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks a reviewer as approved by canonical reviewer key.
    ///
    /// Unknown names are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let mut consensus = ReviewConsensus::new();
    /// consensus.approve("reviewer");
    /// assert_eq!(consensus.approval_count(), 1);
    /// ```
    pub fn approve(&mut self, reviewer_name: &str) {
        match reviewer_name {
            "reviewer" => self.reviewer = true,
            "physics_reviewer" => self.physics_reviewer = true,
            "intent_reviewer" => self.intent_reviewer = true,
            "compliance_reviewer" => self.compliance_reviewer = true,
            _ => {} // Unknown reviewer, ignore
        }
    }

    /// Clears all approvals.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let mut consensus = ReviewConsensus::new();
    /// consensus.approve("reviewer");
    /// consensus.reset();
    /// assert_eq!(consensus.approval_count(), 0);
    /// ```
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns `true` when every required reviewer has approved.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let mut consensus = ReviewConsensus::new();
    /// for reviewer in ["reviewer", "physics_reviewer", "intent_reviewer", "compliance_reviewer"] {
    ///     consensus.approve(reviewer);
    /// }
    /// assert!(consensus.has_consensus());
    /// ```
    pub fn has_consensus(&self) -> bool {
        self.reviewer && self.physics_reviewer && self.intent_reviewer && self.compliance_reviewer
    }

    /// Lists reviewers that already approved the current draft.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let mut consensus = ReviewConsensus::new();
    /// consensus.approve("reviewer");
    /// assert_eq!(consensus.approved_list(), vec!["reviewer"]);
    /// ```
    pub fn approved_list(&self) -> Vec<&str> {
        let mut list = Vec::new();
        if self.reviewer {
            list.push("reviewer");
        }
        if self.physics_reviewer {
            list.push("physics_reviewer");
        }
        if self.intent_reviewer {
            list.push("intent_reviewer");
        }
        if self.compliance_reviewer {
            list.push("compliance_reviewer");
        }
        list
    }

    /// Lists reviewers that still need to approve.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let consensus = ReviewConsensus::new();
    /// assert!(consensus.pending_list().contains(&"reviewer"));
    /// ```
    pub fn pending_list(&self) -> Vec<&str> {
        let mut list = Vec::new();
        if !self.reviewer {
            list.push("reviewer");
        }
        if !self.physics_reviewer {
            list.push("physics_reviewer");
        }
        if !self.intent_reviewer {
            list.push("intent_reviewer");
        }
        if !self.compliance_reviewer {
            list.push("compliance_reviewer");
        }
        list
    }

    /// Returns the number of approvals currently recorded.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::ReviewConsensus;
    ///
    /// let mut consensus = ReviewConsensus::new();
    /// consensus.approve("reviewer");
    /// assert_eq!(consensus.approval_count(), 1);
    /// ```
    pub fn approval_count(&self) -> usize {
        self.approved_list().len()
    }
}

/// Shared blackboard state consumed by all specialists.
///
/// In each turn, an agent receives a full [`GlobalContext`] snapshot and returns an
/// updated one in its [`TurnResult`], preventing hidden state and out-of-band coupling.
///
/// # Fields
///
/// *   `objective`: The original "North Star" goal (e.g., "Design a mars rover"). Never changes.
/// *   `conversation_history`: The "Memory". A shared log of what every agent has said or done.
/// *   `artifacts`: The "Inventory". Files, scripts, or data generated so far.
/// *   `review_consensus`: Tracks which reviewers have approved the current code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalContext {
    /// The high-level user objective (e.g., "Create a donut").
    pub objective: String,
    /// A log of the conversation and actions taken so far.
    pub conversation_history: Vec<String>,
    /// The current state of generated artifacts.
    pub artifacts: Vec<Artifact<crate::types::Blueprint>>, // Simplified for now
    /// The number of steps taken in the current mission.
    pub step_count: u32,
    /// Tracks which reviewers have approved the current code.
    pub review_consensus: ReviewConsensus,
}

impl GlobalContext {
    /// Creates a new context initialized with `objective`.
    ///
    /// # Examples
    ///
    /// ```
    /// use build123d_cad::agentic::GlobalContext;
    ///
    /// let ctx = GlobalContext::new("Design a pressure vessel".to_string());
    /// assert_eq!(ctx.step_count, 0);
    /// assert_eq!(ctx.conversation_history.len(), 0);
    /// ```
    pub fn new(objective: String) -> Self {
        Self {
            objective,
            conversation_history: Vec::new(),
            artifacts: Vec::new(),
            step_count: 0,
            review_consensus: ReviewConsensus::new(),
        }
    }
}

/// Transition signal returned by an agent turn.
///
/// This enum is the protocol boundary between agent logic and the orchestrator loop.
#[derive(Debug)]
pub enum TurnResult {
    /// The agent is not done and wishes to continue working (e.g., thinking, multi-step reasoning).
    ///
    /// # Action
    /// The Orchestrator will call the **same agent** again in the next step.
    KeepWorking {
        /// The agent's thought or reasoning.
        thought: String,
        /// The updated context (with new history/artifacts).
        new_context: GlobalContext,
    },
    /// The agent explicitly calls a tool (runtime dispatch).
    ///
    /// # Action
    /// The Orchestrator executes the tool and adds the result to the history,
    /// then calls the **same agent** again.
    #[allow(dead_code)]
    CallTool {
        /// The name of the tool to call.
        tool_name: String,
        /// The arguments for the tool.
        args: String,
        /// The updated context.
        new_context: GlobalContext,
    },
    /// The agent delegates control to another agent.
    ///
    /// # Action
    /// The Orchestrator switches context to the `target_agent`.
    Delegate {
        /// The name of the target agent (e.g., "researcher").
        /// MUST match a key in the Orchestrator's agent map.
        target_agent: String, // e.g., "coder", "reviewer"
        /// The instruction for the target agent (e.g., "Verify this calculation").
        instruction: String, // "I found the docs, now write the code."
        /// The updated context.
        new_context: GlobalContext,
    },
    /// The mission is complete.
    ///
    /// # Action
    /// The Orchestrator terminates the loop and returns the result.
    FinalResult(String),
}

// use crate::agentic::errors::AgentError;

/// Trait implemented by every specialist agent.
///
/// A specialist owns a role-specific policy and toolset, but always communicates through
/// the shared [`GlobalContext`] and [`TurnResult`] protocol.
#[async_trait]
pub trait Specialist: Send + Sync {
    /// Returns the unique name of the agent (e.g., "supervisor").
    ///
    /// This is used for routing delegation requests in [`crate::agentic::Orchestrator`].
    fn name(&self) -> &str;

    /// Executes one turn of the agent's logic.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The current global context (history, objective, artifacts).
    ///
    /// # Returns
    ///
    /// - `Ok(TurnResult)` with the next transition decision.
    /// - `Err(AgentError)` when prompt/tool execution fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use build123d_cad::agentic::{GlobalContext, Specialist};
    ///
    /// # async fn demo(agent: &dyn Specialist) {
    /// let ctx = GlobalContext::new("Design a finned heat sink".to_string());
    /// let _result = agent.run_turn(ctx).await;
    /// # }
    /// ```
    async fn run_turn(&self, ctx: GlobalContext) -> Result<TurnResult, AgentError>;
}
