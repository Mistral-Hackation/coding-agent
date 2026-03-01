use anyhow::{Context, Result};
use build123d_cad::agentic::agents::{
    Coder, ComplianceReviewer, IntentReviewer, OilGasReviewer, PhysicsReviewer, Researcher,
    Reviewer, Supervisor, SupplyReviewer,
};
use build123d_cad::agentic::{Orchestrator, Specialist};
use build123d_cad::telemetry;
use rig::client::CompletionClient;
use rig::providers::anthropic;
use rig::telemetry::SpanCombinator;
use std::collections::HashMap;
use uuid::Uuid;

const OTEL_SERVICE_NAME: &str = "build123d-cad";

#[tokio::main]
async fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|info| {
        println!("🔥 PANIC: {:?}", info);
    }));

    let telemetry =
        telemetry::init_telemetry(OTEL_SERVICE_NAME).context("failed to initialize telemetry")?;

    // 1. Setup Environment
    let api_key =
        std::env::var("AZURE_API_KEY").context("AZURE_API_KEY environment variable is not set")?;
    let endpoint = std::env::var("AZURE_EXISTING_AIPROJECT_ENDPOINT")
        .context("AZURE_EXISTING_AIPROJECT_ENDPOINT environment variable is not set")?;
    // 2. Initialize AI Client (Azure MaaS)
    // Note: Explicit type annotation is required when overriding base_url
    let client: anthropic::Client<reqwest::Client> = anthropic::Client::builder()
        .api_key(&api_key)
        .base_url(&endpoint)
        .build()
        .context("Failed to build Anthropic client")?;
    let model = client.completion_model("claude-opus-4-5");
    let model_name = "claude-opus-4-5";

    // 3. Create Agents
    let supervisor = Supervisor::new(model.clone());
    let researcher = Researcher::new(model.clone());
    let coder = Coder::new(model.clone());
    let reviewer = Reviewer::new(model.clone());
    let physics_reviewer = PhysicsReviewer::new(model.clone());
    let intent_reviewer = IntentReviewer::new(model.clone());
    let oilgas_reviewer = OilGasReviewer::new(model.clone());
    let compliance_reviewer = ComplianceReviewer::new(model.clone());
    let supply_reviewer = SupplyReviewer::new(model.clone());

    // 4. Register Agents
    let mut agents: HashMap<String, Box<dyn Specialist>> = HashMap::new();
    agents.insert(supervisor.name().to_string(), Box::new(supervisor));
    agents.insert(researcher.name().to_string(), Box::new(researcher));
    agents.insert(coder.name().to_string(), Box::new(coder));
    agents.insert(reviewer.name().to_string(), Box::new(reviewer));
    agents.insert(
        physics_reviewer.name().to_string(),
        Box::new(physics_reviewer),
    );
    agents.insert(
        intent_reviewer.name().to_string(),
        Box::new(intent_reviewer),
    );
    agents.insert(
        oilgas_reviewer.name().to_string(),
        Box::new(oilgas_reviewer),
    );
    agents.insert(
        compliance_reviewer.name().to_string(),
        Box::new(compliance_reviewer),
    );
    agents.insert(
        supply_reviewer.name().to_string(),
        Box::new(supply_reviewer),
    );

    // 5. Initialize Orchestrator
    // 5. Initialize Orchestrator
    // Step limit is configurable via AGENTIC_MAX_STEPS.
    let orchestrator = Orchestrator::new(agents, agentic_max_steps());

    // 6. Run Mission - Choose one of these example prompts:

    // Project name for organized output (optional)
    let project_name = Some("threaded_fitting_20x0.5".to_string());

    // Example 1: Procedural Planet
    // let mission = "Find a tutorial for a procedural planet in Blender and write the Python script.";
    // let project_name = Some("procedural_planet".to_string());

    // Example 2: Electrical Cabinet (Industrial Design)
    // let mission = "Create an electrical cabinet in Blender with stainless steel 316L material, \
    //                similar to industrial welding cabinets. Include door hinges, handle, and ventilation slots.";
    // let project_name = Some("electrical_cabinet".to_string());

    // Example 3: Threaded Fitting Adapter (Oil & Gas)
    let mission = "Create a threaded fitting adapter with metal reinforcing ring in Blender. \
                   Size: 20x1/2\". Dimensions: L1=16mm (non-threaded length), L2=15mm (threaded length), \
                   E=30mm (threaded diameter), Z=26mm (L1 + thicker section). Total Length = Z + L2 = 41mm. \
                   Include realistic thread modeling and metallic material.";

    println!("🚀 Starting Agentic Workflow...");
    let request_id = Uuid::new_v4().to_string();
    let mission_span = tracing::info_span!(
        "rig_build123d_request",
        request_id = %request_id,
        gen_ai_provider_name = "anthropic",
        gen_ai_system = "anthropic.azure",
        gen_ai_operation_name = "chat.completion",
        gen_ai_request_model = %model_name,
        run_entrypoint = "agentic_workflow",
        objective_len = mission.len()
    );
    mission_span.record_model_input(&serde_json::json!({
        "objective_preview": mission.chars().take(220).collect::<String>(),
        "objective_len": mission.len(),
    }));
    let _guard = mission_span.enter();

    let output = orchestrator
        .run_with_project(mission.to_string(), project_name)
        .await?;
    mission_span.record_model_output(&serde_json::json!({
        "status": "ok",
        "result_len": output.len(),
    }));

    println!("✅ Mission Complete");
    telemetry.shutdown().context("failed to flush telemetry")?;
    Ok(())
}

fn agentic_max_steps() -> u32 {
    std::env::var("AGENTIC_MAX_STEPS")
        .ok()
        .and_then(|raw| raw.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(50)
}
