// cli/src/main.rs
use core::App;
use demog::{DemogPlugin, DemogSystem};
use econ::{EconPlugin, EconSystem};
use gov::{GovPlugin, GovSystem};
use models::{DemogState, EconState, GovState};
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use util::fmt_currency;

mod util;

fn main() {
    let mut app = App::new();

    // 1. Load system plugin rules once
    app.add_plugin(&EconPlugin);
    app.add_plugin(&GovPlugin);
    app.add_plugin(&DemogPlugin);

    // 2. Register system archetypes precisely ONCE into the scheduler
    app.scheduler.add_system("econ", Box::new(EconSystem));
    app.scheduler.add_system("gov", Box::new(GovSystem));
    app.scheduler.add_system("demog", Box::new(DemogSystem));

    // 3. Open and ingest hierarchical simulation configuration asset
    let mut file = File::open("scenarios/simulation_config.json")
        .expect("Failed to locate simulation_config.json");
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let root_data: Value = serde_json::from_str(&json_str).expect("Failed to parse config schema");

    // Helper macro to generate static leaking string identifiers for nested keys safely
    fn intern_key(prefix: &str, component: &str) -> &'static str {
        Box::leak(format!("{}:{}", prefix, component).into_boxed_str())
    }

    // Recursive function to step down through nested regional geographical branches
    fn parse_region_node(app: &mut App, current_prefix: &str, node: &Value) {
        if let Some(components) = node.get("components") {
            if let Some(econ_val) = components.get("econ") {
                let state: EconState = serde_json::from_value(econ_val.clone()).unwrap();
                app.state.insert(intern_key(current_prefix, "econ"), state);
            }
            if let Some(gov_val) = components.get("gov") {
                let state: GovState = serde_json::from_value(gov_val.clone()).unwrap();
                app.state.insert(intern_key(current_prefix, "gov"), state);
            }
            if let Some(demog_val) = components.get("demog") {
                let state: DemogState = serde_json::from_value(demog_val.clone()).unwrap();
                app.state.insert(intern_key(current_prefix, "demog"), state);
            }
        }

        // Parse child geographies recursively (e.g., "us" -> "us:los_angeles")
        if let Some(sub_regions) = node.get("sub_regions").and_then(|sr| sr.as_object()) {
            for (sub_name, sub_node) in sub_regions {
                let next_prefix = format!("{}:{}", current_prefix, sub_name);
                parse_region_node(app, &next_prefix, sub_node);
            }
        }
    }

    if let Some(regions) = root_data.get("regions").and_then(|r| r.as_object()) {
        for (region_name, region_node) in regions {
            parse_region_node(&mut app, region_name, region_node);
        }
    }

    println!("[cli] Geographical tree inflated into Double-Buffered parallel states.");
    println!("[cli] Launching execution loops...");
    app.run(20 * 12, sdk::TimeGranularity::Monthly);

    app.summarize_state();
}
