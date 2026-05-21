use clap::Parser;
use cli_args::CliArgs;
use log::{error, info, warn};
use psychohistory_core::init_logger;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

mod cli_args;
mod result_output;
mod scenario;
// logging is now provided by core
mod util;

// Use the core simulation utility for region traversal
use psychohistory_core::simulation::simulate_region_tree;

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

    // Load scenario file using helper
    let root_data = match scenario::load_scenario(&args.scenario_dir) {
        Ok(val) => val,
        Err(msg) => {
            error!("{}", msg);
            return;
        }
    };

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
