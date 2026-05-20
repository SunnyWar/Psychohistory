//! Tests for configuration parsing and validation, focusing on the 'us' region bug.

use core::config::SimulationConfig;
use serde_json::Value;
use std::fs;

#[test]
fn test_us_region_config_parsing() {
    // Load the scenario config JSON
    let data = fs::read_to_string("../../Psychohistory/scenarios/simulation_config.json").expect(
        "Failed to read ../../Psychohistory/scenarios/simulation_config.json (run from core/tests)",
    );
    let root: Value = serde_json::from_str(&data).expect("Invalid JSON");
    let us = &root["regions"]["us"];
    // Try both top-level and components-nested simulation_parameters
    let sim_params = us
        .get("simulation_parameters")
        .or_else(|| {
            us.get("components")
                .and_then(|c| c.get("simulation_parameters"))
        })
        .expect("No simulation_parameters found for 'us'");
    // Try to parse as SimulationConfig
    let config: Result<SimulationConfig, _> = serde_json::from_value(sim_params.clone());
    assert!(
        config.is_ok(),
        "SimulationConfig failed to parse for 'us': {config:?}"
    );
}

#[test]
fn test_all_regions_config_parsing() {
    let data = fs::read_to_string("../../Psychohistory/scenarios/simulation_config.json").expect(
        "Failed to read ../../Psychohistory/scenarios/simulation_config.json (run from core/tests)",
    );
    let root: Value = serde_json::from_str(&data).expect("Invalid JSON");
    let regions = root["regions"].as_object().expect("No regions object");
    for (name, region) in regions {
        let sim_params = region.get("simulation_parameters").or_else(|| {
            region
                .get("components")
                .and_then(|c| c.get("simulation_parameters"))
        });
        if let Some(params) = sim_params {
            let config: Result<SimulationConfig, _> = serde_json::from_value(params.clone());
            assert!(
                config.is_ok(),
                "SimulationConfig failed to parse for region '{name}': {config:?}"
            );
        }
    }
}
