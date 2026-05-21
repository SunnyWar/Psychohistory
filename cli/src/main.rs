use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
mod cli_args;
// logging is now provided by core
mod util;

use psychohistory_core::run_experiment;

use cli_args::CliArgs;
use log::{debug, error, info, warn};
use psychohistory_core::init_logger;

fn main() {
    let args = CliArgs::parse();
    if std::env::args().len() == 1 {
        // No params: print help and exit
        <CliArgs as clap::CommandFactory>::command()
            .print_help()
            .unwrap();
        println!("");
        return;
    }
    init_logger(&args.log_dir, "psychohistory", args.verbose);
    info!("Simulation CLI started");

    // Locate scenario file
    let scenario_path = format!("{}/simulation_config.json", args.scenario_dir);
    let mut file = match File::open(&scenario_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to locate {}: {}", scenario_path, e);
            return;
        }
    };
    let mut json_str = String::new();
    if let Err(e) = file.read_to_string(&mut json_str) {
        error!("Failed to read {}: {}", scenario_path, e);
        return;
    }

    let root_data: Value = match serde_json::from_str(&json_str) {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to parse config schema: {}", e);
            return;
        }
    };

    // Recursively traverse regions and run experiment for each
    fn simulate_region_tree(region_name: &str, node: &Value, years: usize, runs: usize) {
        info!("Simulating region: {} ({} runs)", region_name, runs);
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
            let plugins: Vec<Box<dyn psychohistory_core::simulation::SimulationPlugin>> = vec![];
            let result = run_experiment(&system, years, &config, &plugins, runs);
            info!("Completed region: {}", region_name);
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
            warn!(
                "Skipping region '{}' due to missing or invalid system/config.",
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

    match root_data.get("regions").and_then(|r| r.as_object()) {
        Some(regions) if !regions.is_empty() => {
            info!("Found {} regions in config.", regions.len());
            for (region_name, region_node) in regions {
                simulate_region_tree(region_name, region_node, args.years, args.runs);
            }
        }
        Some(_) => {
            warn!("No regions found in simulation_config.json.");
        }
        None => {
            error!("'regions' key missing or not an object in simulation_config.json.");
        }
    }
}
