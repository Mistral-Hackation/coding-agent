//! Orchestration engine for the multi-agent supervisor loop.
//!
//! [`Orchestrator`] owns a registry of specialists and repeatedly routes control based on
//! each turn's [`TurnResult`].

use super::{GlobalContext, Specialist, TurnResult};
use crate::infra::GitJournal;
// use anyhow::Result; // Removed in favor of strict AgentError
use rig::telemetry::SpanCombinator;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use uuid::Uuid;

/// State machine that drives end-to-end multi-agent CAD generation.
///
/// Conceptually, this is an event loop with explicit handoffs:
///
/// 1. Pull current agent by name.
/// 2. Call [`Specialist::run_turn`].
/// 3. Apply the returned [`TurnResult`] transition.
/// 4. Repeat until completion or step limit exhaustion.
///
/// Successful completion writes generated code into `.output/<project>/` and snapshots
/// progress with [`GitJournal`].
///
/// # Examples
///
/// ```no_run
/// use build123d_cad::agentic::{Orchestrator, Specialist};
/// use build123d_cad::agentic::agents::Supervisor;
/// use std::collections::HashMap;
/// use rig::providers::anthropic;
/// use rig::client::ProviderClient;
/// use rig::client::CompletionClient;
///
/// #[tokio::main]
/// async fn main() {
///     // 1. Setup Agents
///     let client = anthropic::Client::from_env();
///     let model = client.completion_model("claude-3-opus-20240229");
///     
///     let mut agents: HashMap<String, Box<dyn Specialist>> = HashMap::new();
///     agents.insert("supervisor".to_string(), Box::new(Supervisor::new(model)));
///
///     // 2. Create Orchestrator
///     let orchestrator = Orchestrator::new(agents, 10);
///
///     // 3. Run Mission
///     let result = orchestrator.run("Make a blue cube".to_string()).await;
///     
///     match result {
///         Ok(code) => println!("Success! Code generated."),
///         Err(e) => eprintln!("Mission failed: {}", e),
///     }
/// }
/// ```
pub struct Orchestrator {
    agents: HashMap<String, Box<dyn Specialist>>,
    max_steps: u32,
}

impl Orchestrator {
    /// Creates a new orchestrator.
    ///
    /// # Arguments
    ///
    /// * `agents` - Registry keyed by canonical agent names (for example, `"supervisor"`).
    /// * `max_steps` - Hard ceiling that prevents infinite loops.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use build123d_cad::agentic::{Orchestrator, Specialist};
    /// use std::collections::HashMap;
    ///
    /// let agents: HashMap<String, Box<dyn Specialist>> = HashMap::new();
    /// let _orchestrator = Orchestrator::new(agents, 50);
    /// ```
    pub fn new(agents: HashMap<String, Box<dyn Specialist>>, max_steps: u32) -> Self {
        Self { agents, max_steps }
    }

    /// Runs a mission using an auto-generated project directory name.
    ///
    /// This is a convenience wrapper around [`Orchestrator::run_with_project`] with
    /// `project_name = None`.
    ///
    /// # Errors
    ///
    /// Returns [`super::errors::AgentError`] if startup fails or the step limit is reached.
    #[tracing::instrument(
        name = "rig_orchestrator_request",
        skip(self, initial_objective),
        fields(
            max_steps = self.max_steps,
            objective_len = initial_objective.len(),
            request_id = %uuid::Uuid::new_v4().to_string(),
            gen_ai_provider_name = "anthropic",
            gen_ai_system = "anthropic.azure",
            gen_ai_operation_name = "agent.workflow",
            gen_ai_request_model = %llm_model_name(),
            gen_ai_error_type = "none"
        )
    )]
    pub async fn run(
        &self,
        initial_objective: String,
    ) -> Result<String, super::errors::AgentError> {
        tracing::Span::current().record_model_input(&serde_json::json!({
            "objective_preview": truncate_for_telemetry(&initial_objective, 180),
            "objective_len": initial_objective.len(),
            "gen_ai_provider_name": "anthropic",
            "gen_ai_system": "anthropic.azure",
            "gen_ai_request_model": llm_model_name(),
        }));
        tracing::Span::current().record("workflow_stage", "run_entry");
        self.run_with_project(initial_objective, None).await
    }

    /// Runs a mission with an optional project name for deterministic output paths.
    ///
    /// This method initializes [`GlobalContext`], executes the supervisor loop, writes
    /// periodic snapshots, and returns the final generated script text when successful.
    ///
    /// # Arguments
    ///
    /// * `initial_objective` - The user's prompt (e.g., "Create a procedural skyscraper").
    /// * `project_name` - Optional project name. If None, uses `output_<timestamp>`.
    ///
    /// # Output Folder Structure
    ///
    /// ```text
    /// .output/
    /// └── my_project/           # or output_20240124_123456/
    ///     ├── .git/             # Git repository
    ///     ├── step_0005_context.json
    ///     ├── step_0005_code.py
    ///     └── generated_script_*.py
    /// ```
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: final generated script (usually Python code).
    /// - `Err(AgentError)`: startup failure or unresolved mission.
    ///
    /// # Errors
    ///
    /// Returns [`super::errors::AgentError::FileSystemError`] when output directory creation
    /// fails, and [`super::errors::AgentError::MaxStepsReached`] when no terminal result is
    /// produced before `max_steps`.
    ///
    /// # Panics
    ///
    /// This function does not intentionally panic.
    #[tracing::instrument(
        name = "agent_orchestrator",
        skip(self, initial_objective, project_name),
        fields(
            max_steps = self.max_steps,
            objective_len = initial_objective.len(),
            request_id = %uuid::Uuid::new_v4().to_string(),
            gen_ai_provider_name = "anthropic",
            gen_ai_system = "anthropic.azure",
            gen_ai_operation_name = "agent.workflow",
            gen_ai_request_model = %llm_model_name(),
            project_name = %project_name.clone().unwrap_or_else(|| "auto".to_string()),
            gen_ai_error_type = "none"
        )
    )]
    pub async fn run_with_project(
        &self,
        initial_objective: String,
        project_name: Option<String>,
    ) -> Result<String, super::errors::AgentError> {
        let project_label = project_name.as_deref().unwrap_or("auto").to_string();
        let run_id = Uuid::new_v4().to_string();
        let orchestrator_span = tracing::Span::current();
        orchestrator_span.record_model_input(&serde_json::json!({
            "objective_preview": truncate_for_telemetry(&initial_objective, 220),
            "objective_len": initial_objective.len(),
            "project_name": project_label,
            "max_steps": self.max_steps,
            "pipeline_id": &run_id,
            "gen_ai_provider_name": "anthropic",
            "gen_ai_system": "anthropic.azure",
            "gen_ai_request_model": llm_model_name(),
        }));
        orchestrator_span.record("workflow_stage", "orchestrator_loop");

        // Create structured output folder
        let output_dir = generate_output_dir(project_name);

        fs::create_dir_all(&output_dir)
            .map_err(|e| super::errors::AgentError::FileSystemError(e.to_string()))?;
        println!("📂 Project folder: {}", output_dir);

        // Initialize Git repository
        let git = GitJournal::new(&output_dir);

        let mut context = GlobalContext::new(initial_objective);
        let mut tool_usage_by_agent: BTreeMap<String, BTreeSet<String>> = self
            .agents
            .keys()
            .map(|name| (name.clone(), BTreeSet::new()))
            .collect();
        let mut tool_events: Vec<String> = Vec::new();
        let mut unsupported_tool_events: Vec<String> = Vec::new();
        // Start with Supervisor
        let mut current_agent_name = "supervisor".to_string();

        while context.step_count < self.max_steps {
            let step_span = tracing::info_span!(
                "agent_turn",
                step = context.step_count,
                agent = %current_agent_name,
                history_len = context.conversation_history.len(),
                artifact_count = context.artifacts.len(),
                workflow_stage = "agent_turn",
                gen_ai_provider_name = "anthropic",
                gen_ai_system = "anthropic.azure",
                gen_ai_operation_name = "agent.loop_step",
                gen_ai_request_model = %llm_model_name()
            );
            step_span.record_model_input(&serde_json::json!({
                "objective_len": context.objective.len(),
                "history_len": context.conversation_history.len(),
                "artifact_count": context.artifacts.len(),
                "gen_ai_agent": current_agent_name,
                "gen_ai_operation_name": "agent.loop_step",
                "workflow_stage": "agent_turn",
            }));
            let _step_guard = step_span.enter();

            println!(
                "--- Step {}: Agent [{}] ---",
                context.step_count, current_agent_name
            );
            tracing::info!(
                step = context.step_count,
                agent = %current_agent_name,
                "Starting agent turn"
            );

            // Try to get the agent, handle not found gracefully
            let agent = match self.agents.get(&current_agent_name) {
                Some(a) => a,
                None => {
                    let error_msg = format!(
                        "Agent '{}' not found. Available: {:?}. Returning to supervisor.",
                        current_agent_name,
                        self.agents.keys().collect::<Vec<_>>()
                    );
                    println!("   ⚠️ {}", error_msg);
                    context
                        .conversation_history
                        .push(format!("SYSTEM: {}", error_msg));
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "missing_agent_fallback",
                        "next_agent": "supervisor",
                        "status": "error",
                        "gen_ai_error_type": "agent_not_found",
                    }));
                    step_span.record("transition", "missing_agent_fallback");
                    current_agent_name = "supervisor".to_string();
                    context.step_count += 1;
                    continue;
                }
            };

            let result = match agent.run_turn(context.clone()).await {
                Ok(res) => res,
                Err(e) => {
                    let error_msg = format!("Agent '{}' failed: {}", current_agent_name, e);
                    println!("   ❌ {}", error_msg);
                    tracing::error!(agent = %current_agent_name, error = %e, "Agent turn failed");
                    context
                        .conversation_history
                        .push(format!("SYSTEM: {}", error_msg));
                    // Recover by handing back to Supervisor logic, or just continuing the loop
                    // Assuming the Supervisor will see the error in history next turn.
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "agent_error",
                        "status": "error",
                        "gen_ai_error_type": "agent_turn_failure",
                        "next_agent": "supervisor",
                    }));
                    step_span.record("error", "agent_turn_failure");
                    step_span.record("transition", "agent_error");
                    current_agent_name = "supervisor".to_string();
                    continue;
                }
            };

            match result {
                TurnResult::KeepWorking {
                    thought,
                    new_context,
                } => {
                    println!(
                        "   >> Thinking: {}",
                        thought.lines().next().unwrap_or("...")
                    );
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "keep_working",
                        "status": "working",
                        "thought_preview": truncate_for_telemetry(&thought, 180),
                    }));
                    step_span.record("transition", "keep_working");
                    context = new_context;
                }
                TurnResult::Delegate {
                    target_agent,
                    instruction,
                    new_context,
                } => {
                    println!("   >> DELEGATING to [{}]", target_agent);
                    // Show instruction with proper formatting (first 200 chars on first line, then full)
                    if instruction.len() > 200 {
                        println!("   >> Instruction (truncated): {}...", &instruction[..200]);
                        println!("   >> Full instruction in conversation history.");
                    } else {
                        println!("   >> Instruction: {}", instruction);
                    }
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "delegate",
                        "status": "delegating",
                        "target_agent": &target_agent,
                        "instruction_preview": truncate_for_telemetry(&instruction, 220),
                    }));
                    step_span.record("transition", "delegate");
                    context = new_context;
                    current_agent_name = target_agent;
                }
                TurnResult::CallTool {
                    tool_name,
                    args,
                    new_context,
                } => {
                    let normalized_tool_name = tool_name.trim().to_string();
                    tool_usage_by_agent
                        .entry(current_agent_name.clone())
                        .or_default()
                        .insert(normalized_tool_name.clone());
                    tool_events.push(format!(
                        "{} called {} ({})",
                        current_agent_name,
                        normalized_tool_name,
                        truncate_for_telemetry(&args, 120)
                    ));

                    let is_disabled_web_search = normalized_tool_name
                        .eq_ignore_ascii_case("web_search")
                        || normalized_tool_name.eq_ignore_ascii_case("websearch");
                    println!("   🛠️ Tool Call: {} args='{}'", tool_name, args);
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "call_tool",
                        "status": "tool_in_progress",
                        "tool_name": &tool_name,
                        "args_preview": truncate_for_telemetry(&args, 160),
                        "gen_ai_operation_name": "agent.tool.use",
                        "tool_enabled": !is_disabled_web_search,
                    }));
                    step_span.record("transition", "call_tool");

                    let tool_span = if is_disabled_web_search {
                        tracing::info_span!(
                            "tool.unsupported",
                            tool_name = %normalized_tool_name,
                            agent = %current_agent_name,
                            gen_ai_operation_name = "agent.tool.use",
                        )
                    } else {
                        tracing::info_span!(
                            "tool.agent_call",
                            tool_name = %normalized_tool_name,
                            agent = %current_agent_name,
                            gen_ai_operation_name = "agent.tool.use",
                        )
                    };
                    tool_span.record_model_input(&serde_json::json!({
                        "agent": &current_agent_name,
                        "tool_name": &normalized_tool_name,
                        "args_len": args.len(),
                    }));
                    let _tool_guard = tool_span.enter();

                    if is_disabled_web_search {
                        let msg = format!(
                            "Tool '{}' is disabled for this runtime. Use KnowledgeBase and Build123dDocs tools instead.",
                            normalized_tool_name
                        );
                        tool_span.record("status", "disabled");
                        tool_span.record("gen_ai_error_type", "disabled_tool");
                        tracing::warn!(
                            agent = %current_agent_name,
                            tool = %normalized_tool_name,
                            "Blocked web-search tool call for this runtime"
                        );
                        unsupported_tool_events.push(msg.clone());
                        context
                            .conversation_history
                            .push(format!("Tool Output (disabled): {}", msg));
                    } else {
                        tool_span.record("status", "handled_by_rig");
                        tool_span.record("gen_ai_error_type", "none");
                    }
                    context = new_context;
                }
                TurnResult::FinalResult(output) => {
                    step_span.record_model_output(&serde_json::json!({
                        "transition": "final_result",
                        "status": "ok",
                        "result_len": output.len(),
                    }));
                    step_span.record("transition", "final_result");
                    step_span.record("status", "ok");
                    // Sanitize the output: strip any trailing non-Python text
                    // (LLM sometimes appends review commentary after code)
                    let sanitized_code = sanitize_python_output(&output);

                    // Save the final result to the project output folder
                    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                    let filename = format!("{}/generated_script_{}.py", output_dir, timestamp);

                    match fs::write(&filename, &sanitized_code) {
                        Ok(_) => {
                            println!("\n📁 Script saved to: {}", filename);

                            // Commit the new script to the git repository
                            let short_objective: String =
                                context.objective.chars().take(50).collect();
                            let commit_msg = format!(
                                "Generated script for: {}{}",
                                short_objective,
                                if context.objective.len() > 50 {
                                    "..."
                                } else {
                                    ""
                                }
                            );
                            git.snapshot("Orchestrator", &commit_msg);
                            println!("📦 Committed to Git: [Orchestrator] {}", commit_msg);

                            // Auto-execute the generated Python script
                            // Extract basename since we set current_dir to output_dir
                            // so that output files (.step, .svg) are written there
                            let script_basename = std::path::Path::new(&filename)
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| filename.clone());
                            println!("\n🚀 Auto-executing generated script via uv...");
                            match tokio::process::Command::new("uv")
                                .args(["run", "--with", "build123d", "python3", &script_basename])
                                .current_dir(&output_dir)
                                .output()
                                .await
                            {
                                Ok(exec_result) => {
                                    let stdout = String::from_utf8_lossy(&exec_result.stdout);
                                    let stderr = String::from_utf8_lossy(&exec_result.stderr);

                                    if exec_result.status.success() {
                                        println!("   ✅ Script executed successfully!");
                                        if !stdout.is_empty() {
                                            println!("   📤 Output: {}", stdout.trim());
                                        }

                                        // List generated output files
                                        if let Ok(entries) = fs::read_dir(&output_dir) {
                                            let output_files: Vec<String> = entries
                                                .filter_map(|e| e.ok())
                                                .filter(|e| {
                                                    let name =
                                                        e.file_name().to_string_lossy().to_string();
                                                    name.ends_with(".step")
                                                        || name.ends_with(".stl")
                                                        || name.ends_with(".svg")
                                                })
                                                .map(|e| {
                                                    e.file_name().to_string_lossy().to_string()
                                                })
                                                .collect();

                                            if !output_files.is_empty() {
                                                println!("   📂 Generated files:");
                                                for file in &output_files {
                                                    println!("      • {}/{}", output_dir, file);
                                                }

                                                let has_stl =
                                                    output_files.iter().any(|f| f.ends_with(".stl"));

                                                // Generate interactive 3D viewer if STL exists
                                                match crate::viewer::generate_and_open(
                                                    std::path::Path::new(&output_dir),
                                                ) {
                                                    Ok(viewer_path) => {
                                                        println!(
                                                            "   🌐 3D Viewer: {}",
                                                            viewer_path.display()
                                                        );
                                                    }
                                                    Err(crate::viewer::ViewerError::NoStlFound) => {
                                                        if !has_stl {
                                                            println!(
                                                                "   ⚠️ 3D viewer was not generated because no STL file was found."
                                                            );
                                                        }
                                                    }
                                                    Err(e) => {
                                                        println!(
                                                            "   ⚠️ Viewer generation failed: {}",
                                                            e
                                                        );
                                                    }
                                                }
                                            }
                                        }

                                        // Commit execution output to git
                                        git.snapshot("Orchestrator", "Auto-execution output");
                                    } else {
                                        println!(
                                            "   ⚠️ Script execution failed (exit code: {:?})",
                                            exec_result.status.code()
                                        );
                                        if !stderr.is_empty() {
                                            // Show first 500 chars of stderr
                                            let truncated_err: String =
                                                stderr.chars().take(500).collect();
                                            println!("   ❌ Error: {}", truncated_err);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("   ⚠️ Failed to execute script: {}", e);
                                    println!("   💡 Run manually: python3 {}", filename);
                                }
                            }
                        }
                        Err(e) => println!("\n⚠️ Failed to save script: {}", e),
                    }

                    orchestrator_span.record_model_output(&serde_json::json!({
                        "status": "ok",
                        "steps_taken": context.step_count,
                        "result_len": output.len(),
                        "saved_file": filename,
                    }));
                    return Ok(output);
                }
            }

            context.step_count += 1;

            // Periodic snapshot every 5 steps for observability
            if context.step_count.is_multiple_of(5) && context.step_count > 0 {
                // Extract any generated code from conversation history
                let generated_code = context
                    .conversation_history
                    .iter()
                    .rev()
                    .find(|entry| entry.starts_with("GENERATED_CODE:"))
                    .map(|entry| entry.strip_prefix("GENERATED_CODE:\n").unwrap_or(entry));

                // Save current context as JSON for debugging
                let snapshot_file =
                    format!("{}/step_{:04}_context.json", output_dir, context.step_count);
                let snapshot_content = serde_json::json!({
                    "step": context.step_count,
                    "current_agent": current_agent_name,
                    "objective": &context.objective,
                    "history_length": context.conversation_history.len(),
                    "last_3_history": context.conversation_history.iter().rev().take(3).collect::<Vec<_>>(),
                    "artifacts_count": context.artifacts.len(),
                    "has_generated_code": generated_code.is_some(),
                });

                if let Ok(json) = serde_json::to_string_pretty(&snapshot_content)
                    && fs::write(&snapshot_file, &json).is_ok()
                {
                    println!("   📸 Snapshot saved: {}", snapshot_file);
                }

                // Also save the generated Python code if available
                if let Some(code) = generated_code {
                    let code_file =
                        format!("{}/step_{:04}_code.py", output_dir, context.step_count);
                    if fs::write(&code_file, code).is_ok() {
                        println!("   🐍 Code snapshot: {}", code_file);
                    }
                }

                // Commit all snapshots to git
                git.snapshot(
                    &current_agent_name,
                    &format!(
                        "Step {} checkpoint - {}",
                        context.step_count,
                        context.objective.chars().take(30).collect::<String>()
                    ),
                );
            }
        }

        orchestrator_span.record_model_output(&serde_json::json!({
            "status": "max_steps_reached",
            "max_steps": self.max_steps,
            "steps_taken": context.step_count,
            "gen_ai_error_type": "max_steps_reached",
        }));
        orchestrator_span.record("status", "max_steps_reached");
        orchestrator_span.record("gen_ai_error_type", "max_steps_reached");
        Err(super::errors::AgentError::MaxStepsReached(self.max_steps))
    }
}

fn llm_model_name() -> String {
    std::env::var("AZURE_MODEL").unwrap_or_else(|_| "claude-opus-4-5".to_string())
}

/// Sanitizes a string to be a valid folder name.
/// Replaces spaces with underscores and removes special characters.
fn sanitize_folder_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .to_lowercase()
}

fn generate_output_dir(project_name: Option<String>) -> String {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let folder_name = project_name
        .map(|name| format!("{}_{}", sanitize_folder_name(&name), timestamp))
        .unwrap_or_else(|| format!("output_{}", timestamp));
    format!(".output/{}", folder_name)
}

fn truncate_for_telemetry(input: &str, max_chars: usize) -> String {
    let mut preview: String = input.chars().take(max_chars).collect();
    if input.chars().count() > max_chars {
        preview.push_str("...");
    }
    preview
}

/// Strips trailing non-Python text from LLM output.
///
/// LLMs sometimes append review commentary or markdown after the code block.
/// This function extracts only the Python code portion.
fn sanitize_python_output(raw: &str) -> String {
    let trimmed = raw.trim();

    // Strategy 1: If wrapped in markdown code fences, extract the inner block
    if let Some(start_idx) = trimmed.find("```python") {
        let code_start = start_idx + "```python".len();
        if let Some(end_idx) = trimmed[code_start..].find("```") {
            return trimmed[code_start..code_start + end_idx].trim().to_string();
        }
    }
    // Also handle bare ``` fences
    if trimmed.starts_with("```") {
        let code_start = trimmed.find('\n').unwrap_or(3) + 1;
        if let Some(end_idx) = trimmed[code_start..].find("```") {
            return trimmed[code_start..code_start + end_idx].trim().to_string();
        }
    }

    // Strategy 2: Truncate after the last line that looks like Python code
    // Heuristic: if we encounter lines that look like English prose after code,
    // remove them. Look for the last line that ends with Python syntax markers.
    let lines: Vec<&str> = trimmed.lines().collect();
    let mut last_code_line = lines.len();

    for (i, line) in lines.iter().enumerate().rev() {
        let stripped = line.trim();
        // Skip empty lines
        if stripped.is_empty() {
            continue;
        }
        // Lines that look like Python code (contain typical Python markers)
        if stripped.starts_with('#')
            || stripped.starts_with("import ")
            || stripped.starts_with("from ")
            || stripped.starts_with("def ")
            || stripped.starts_with("class ")
            || stripped.starts_with("if ")
            || stripped.starts_with("print(")
            || stripped.ends_with(':')
            || stripped.ends_with(')')
            || stripped.ends_with('\"')
            || stripped.ends_with('\'')
            || stripped.contains('=')
            || stripped.contains('(')
        {
            last_code_line = i + 1;
            break;
        }
    }

    lines[..last_code_line].join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_output_dir_with_name() {
        let dir = generate_output_dir(Some("My Project".to_string()));
        assert!(dir.starts_with(".output/my_project_"));
        // Timestamp is 15 chars (YYYYMMDD_HHMMSS)
        // .output/my_project_ + 15 chars
        assert_eq!(dir.len(), ".output/my_project_".len() + 15);
    }

    #[test]
    fn test_generate_output_dir_no_name() {
        let dir = generate_output_dir(None);
        assert!(dir.starts_with(".output/output_"));
        // .output/output_ + 15 chars
        assert_eq!(dir.len(), ".output/output_".len() + 15);
    }
}
