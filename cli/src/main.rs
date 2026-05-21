use clap::Parser;
use cli_args::CliArgs;
use log::{error, info, warn};
use psychohistory_core::experiment::ExperimentResult;
use psychohistory_core::init_logger;
use psychohistory_core::run_experiment;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

mod cli_args;
mod result_output;
// logging is now provided by core
mod util;

fn main() {
    let args = CliArgs::parse();
    if std::env::args().len() == 1 {
        // No params: print help and exit
        <CliArgs as clap::CommandFactory>::command()
            .print_help()
            .unwrap();
        println!();
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

    // Use the core simulation utility for region traversal
    use psychohistory_core::simulation::simulate_region_tree;

    match root_data.get("regions").and_then(|r| r.as_object()) {
        Some(regions) if !regions.is_empty() => {
            info!("Found {} regions in config.", regions.len());
            for (region_name, region_node) in regions {
                simulate_region_tree(
                    region_name,
                    region_node,
                    args.years,
                    args.runs,
                    &result_output::print_experiment_results,
                );
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
