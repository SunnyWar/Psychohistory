use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to find target dir");
    let dest_dir = target_dir.join(&profile);
    let scenarios_dir = Path::new("scenarios");
    if scenarios_dir.exists() {
        for entry in fs::read_dir(scenarios_dir).expect("Failed to read scenarios dir") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let file_name = path.file_name().unwrap();
                let dest_path = dest_dir.join(file_name);
                fs::copy(&path, &dest_path).expect("Failed to copy file");
            }
        }
    }
}
