use build123d_cad::agentic::agents::{
    Coder, ComplianceReviewer, IntentReviewer, PhysicsReviewer, Researcher, Reviewer, Supervisor,
};
use build123d_cad::agentic::{Orchestrator, Specialist};
use build123d_cad::telemetry;
use clap::Parser;
use rig::client::CompletionClient;
use rig::providers::anthropic;
use rig::telemetry::SpanCombinator;
use std::collections::HashMap;
use uuid::Uuid;

/// build123d CAD Code Generator CLI
#[derive(Parser, Debug)]
#[command(
    version,
    about = "🤖 Multi-Agent Orchestrator for build123d CAD Code Generation",
    long_about = "🤖 Enterprise build123d CAD Code Generator\n\n\
        A Supervisor-Worker State Machine architecture with multi-stage quality review.\n\n\
        • Supervisor - Routes tasks to specialists\n\
        • Researcher - Gathers information from Build123dDocs and local examples\n\
        • Coder - Writes build123d Python CAD scripts\n\
        • Reviewer - Reviews code quality\n\
        • PhysicsReviewer - Reviews geometry and topology accuracy\n\
        • IntentReviewer - Ensures user intent is met\n\
        • ComplianceReviewer - EU regulatory compliance (CE, ATEX, PED) [FINAL GATE]"
)]
struct Args {
    /// Objective for the agents (e.g. "Create a parametric flange model")
    #[arg(num_args = 0..)]
    objectives: Vec<String>,
}

const OTEL_SERVICE_NAME: &str = "build123d-cad";

fn get_env_var(name: &str, description: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| {
        format!(
            "❌ Missing environment variable: {}\n   Description: {}\n",
            name, description
        )
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.objectives.is_empty() {
        println!("Please provide an objective. Usage: build123d_cad 'objective'");
        return Ok(());
    }

    let telemetry = telemetry::init_telemetry(OTEL_SERVICE_NAME)
        .map_err(|e| format!("❌ Failed to initialize telemetry: {}", e))?;
    tracing::info!(
        otel_enabled = telemetry.is_enabled(),
        "Telemetry initialized"
    );

    // Load Azure configuration
    let api_key = get_env_var("AZURE_API_KEY", "Your Azure API key")?;
    let endpoint = get_env_var(
        "AZURE_EXISTING_AIPROJECT_ENDPOINT",
        "Your Azure Anthropic endpoint",
    )?;
    let model = std::env::var("AZURE_MODEL").unwrap_or_else(|_| "claude-opus-4-5".to_string());

    println!("🚀 Multi-Agent Orchestrator for build123d CAD Code Generation");
    println!("   Endpoint: {}", endpoint);
    println!("   Model: {}", model);
    println!(
        "   Agents: Supervisor → Researcher → Coder → Reviewer → PhysicsReviewer → IntentReviewer → ComplianceReviewer\n"
    );

    // Build Anthropic client
    let client: anthropic::Client<reqwest::Client> = anthropic::Client::builder()
        .api_key(&api_key)
        .base_url(&endpoint)
        .build()
        .map_err(|e| format!("❌ Failed to build client: {}", e))?;

    for objective in args.objectives {
        let request_id = Uuid::new_v4().to_string();
        let objective_preview: String = objective.chars().take(180).collect();
        let mission_span = tracing::info_span!(
            "rig_build123d_request",
            request_id = %request_id,
            gen_ai_provider_name = "anthropic",
            gen_ai_system = "anthropic.azure",
            gen_ai_operation_name = "chat.completion",
            gen_ai_request_model = %model,
            gen_ai_error_type = "none",
            run_entrypoint = "cli",
            objective_len = objective.len()
        );
        mission_span.record_model_input(&serde_json::json!({
            "objective_preview": objective_preview,
            "objective_len": objective.len(),
            "workflow": "agentic_cad",
        }));
        let _mission_guard = mission_span.enter();

        println!("\n▶️ Starting Mission: \"{}\"", objective);
        tracing::info!("Starting mission");

        // Initialize all agents
        let completion_model = client.completion_model(&model);
        let supervisor = Supervisor::new(completion_model.clone());
        let researcher = Researcher::new(completion_model.clone());
        let coder = Coder::new(completion_model.clone());
        let reviewer = Reviewer::new(completion_model.clone());
        let physics_reviewer = PhysicsReviewer::new(completion_model.clone());
        let intent_reviewer = IntentReviewer::new(completion_model.clone());
        let compliance_reviewer = ComplianceReviewer::new(completion_model.clone());

        // Register all agents
        let mut agents: HashMap<String, Box<dyn Specialist>> = HashMap::new();
        agents.insert("supervisor".into(), Box::new(supervisor));
        agents.insert("researcher".into(), Box::new(researcher));
        agents.insert("coder".into(), Box::new(coder));
        agents.insert("reviewer".into(), Box::new(reviewer));
        agents.insert("physics_reviewer".into(), Box::new(physics_reviewer));
        agents.insert("intent_reviewer".into(), Box::new(intent_reviewer));
        agents.insert("compliance_reviewer".into(), Box::new(compliance_reviewer));

        let max_steps = agentic_max_steps();
        let orchestrator = Orchestrator::new(agents, max_steps);

        // Run mission
        match orchestrator.run(objective).await {
            Ok(result) => {
                mission_span.record_model_output(&serde_json::json!({
                    "status": "ok",
                    "result_len": result.len(),
                }));
                tracing::info!(result_len = result.len(), "Mission completed");
                println!("✅ Mission Result:\n{}", result);
            }
            Err(e) => {
                mission_span.record_model_output(&serde_json::json!({
                    "status": "error",
                    "error": e.to_string(),
                }));
                tracing::error!(error = %e, "Mission failed");
                eprintln!("❌ Mission Failed: {}", e);
            }
        }
    }

    telemetry
        .shutdown()
        .map_err(|e| format!("❌ Failed to flush telemetry: {}", e))?;

    Ok(())
}

fn agentic_max_steps() -> u32 {
    std::env::var("AGENTIC_MAX_STEPS")
        .ok()
        .and_then(|raw| raw.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(50)
}
