use std::fs;
use serde_json::Value;

#[test]
fn simulation_config_json_is_valid() {
    let path = "scenarios/simulation_config.json";
    let data = fs::read_to_string(path)
        .expect("Failed to read simulation_config.json (check path and existence)");
    serde_json::from_str::<Value>(&data)
        .expect("simulation_config.json is not valid JSON");
}
