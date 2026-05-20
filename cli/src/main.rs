use core::run_experiment;
use core::simulation::run_simulation;
// cli/src/main.rs
use serde_json::Value;
use std::fs::File;
use std::io::Read;

mod util;

fn main() {
    println!("[INFO] Starting simulation CLI...");
    // Open and ingest hierarchical simulation configuration asset
    let mut file = match File::open("scenarios/simulation_config.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[ERROR] Failed to locate simulation_config.json: {}", e);
            return;
        }
    };
    let mut json_str = String::new();
    if let Err(e) = file.read_to_string(&mut json_str) {
        eprintln!("[ERROR] Failed to read simulation_config.json: {}", e);
        return;
    }

    let root_data: Value = match serde_json::from_str(&json_str) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("[ERROR] Failed to parse config schema: {}", e);
            return;
        }
    };

    // Recursively traverse regions and run experiment for each
    fn simulate_region_tree(region_name: &str, node: &Value, years: usize, runs: usize) {
        println!("[INFO] Simulating region: {} ({} runs)", region_name, runs);
        // Try to load GovernanceSystem and SimulationConfig for this region
        // Support both top-level and 'components'-nested fields
        let (system, config) = {
            let system = node
                .get("governance_system")
                .or_else(|| {
                    node.get("components")
                        .and_then(|c| c.get("governance_system"))
                })
                .and_then(|gs| serde_json::from_value(gs.clone()).ok());
            let config = node
                .get("simulation_parameters")
                .or_else(|| {
                    node.get("components")
                        .and_then(|c| c.get("simulation_parameters"))
                })
                .and_then(|sp| serde_json::from_value(sp.clone()).ok());
            (system, config)
        };

        if let (Some(system), Some(config)) = (system, config) {
            let plugins: Vec<Box<dyn core::simulation::SimulationPlugin>> = vec![]; // No plugins for now
            let result = run_experiment(&system, years, &config, &plugins, runs);
            println!("=== Experiment Results for region: {} ===", region_name);
            println!(
                "  [MEAN] Law Quality: {:.3}",
                result.mean.average_law_quality
            );
            println!(
                "  [MEAN] Corruption Level: {:.3}",
                result.mean.average_corruption_level
            );
            println!(
                "  [MEAN] Public Trust: {:.3}",
                result.mean.average_public_trust
            );
            println!(
                "  [MEAN] Crisis Response: {:.3}",
                result.mean.average_crisis_response
            );
            println!(
                "  [MEAN] Adaptability: {:.3}",
                result.mean.average_adaptability
            );
            println!(
                "  [MEAN] Representation Accuracy: {:.3}",
                result.mean.average_representation_accuracy
            );
            println!(
                "  [MEAN] Legislative Speed: {:.3}",
                result.mean.average_legislative_speed
            );
            println!(
                "  [MEAN] Economic Outcome: {:.3}",
                result.mean.average_economic_outcome
            );
            println!(
                "  [MEAN] Composite Score: {:.3}",
                result.mean.average_composite_score
            );
            println!(
                "  [STDDEV] Law Quality: {:.3}",
                result.stddev.average_law_quality
            );
            println!(
                "  [STDDEV] Corruption Level: {:.3}",
                result.stddev.average_corruption_level
            );
            println!(
                "  [STDDEV] Public Trust: {:.3}",
                result.stddev.average_public_trust
            );
            println!(
                "  [STDDEV] Crisis Response: {:.3}",
                result.stddev.average_crisis_response
            );
            println!(
                "  [STDDEV] Adaptability: {:.3}",
                result.stddev.average_adaptability
            );
            println!(
                "  [STDDEV] Representation Accuracy: {:.3}",
                result.stddev.average_representation_accuracy
            );
            println!(
                "  [STDDEV] Legislative Speed: {:.3}",
                result.stddev.average_legislative_speed
            );
            println!(
                "  [STDDEV] Economic Outcome: {:.3}",
                result.stddev.average_economic_outcome
            );
            println!(
                "  [STDDEV] Composite Score: {:.3}",
                result.stddev.average_composite_score
            );
        } else {
            println!(
                "[WARN] Skipping region '{}' due to missing or invalid system/config.",
                region_name
            );
        }

        // Recurse into sub_regions
        if let Some(sub_regions) = node.get("sub_regions").and_then(|sr| sr.as_object()) {
            for (sub_name, sub_node) in sub_regions {
                let next_name = format!("{}:{}", region_name, sub_name);
                simulate_region_tree(&next_name, sub_node, years, runs);
            }
        }
    }

    // Parse command-line args for years and runs
    let args: Vec<String> = std::env::args().collect();
    let years = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    let runs = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    match root_data.get("regions").and_then(|r| r.as_object()) {
        Some(regions) if !regions.is_empty() => {
            println!("[INFO] Found {} regions in config.", regions.len());
            for (region_name, region_node) in regions {
                simulate_region_tree(region_name, region_node, years, runs);
            }
        }
        Some(_) => {
            println!("[WARN] No regions found in simulation_config.json.");
        }
        None => {
            println!("[ERROR] 'regions' key missing or not an object in simulation_config.json.");
        }
    }
}
