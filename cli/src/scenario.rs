#[cfg(test)]
mod tests {
    use super::load_scenario;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_scenario_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("simulation_config.json");
        let mut file = File::create(&file_path).unwrap();
        let json = r#"{ "regions": { "test": {} } }"#;
        file.write_all(json.as_bytes()).unwrap();
        let result = load_scenario(dir.path().to_str().unwrap());
        assert!(result.is_ok());
        let val = result.unwrap();
        assert!(val.get("regions").is_some());
    }

    #[test]
    fn test_load_scenario_missing_file() {
        let dir = tempdir().unwrap();
        let result = load_scenario(dir.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Failed to locate"));
    }

    #[test]
    fn test_load_scenario_invalid_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("simulation_config.json");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"not json").unwrap();
        let result = load_scenario(dir.path().to_str().unwrap());
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .contains("Failed to parse config schema")
        );
    }
}
/// Scenario file loading and validation utilities
use serde_json::Value;
use std::fs::File;
use std::io::Read;

/// Loads and parses the scenario JSON file at the given directory path.
/// Returns the root serde_json::Value on success.
pub fn load_scenario(scenario_dir: &str) -> Result<Value, String> {
    let scenario_path = format!("{}/simulation_config.json", scenario_dir);
    let mut file = File::open(&scenario_path)
        .map_err(|e| format!("Failed to locate {}: {}", scenario_path, e))?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)
        .map_err(|e| format!("Failed to read {}: {}", scenario_path, e))?;
    let root_data: Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse config schema: {}", e))?;
    Ok(root_data)
}
