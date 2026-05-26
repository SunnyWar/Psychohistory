use std::collections::HashMap;
use std::sync::RwLock;

/// InfluenceRegistry manages directed influence weights between string keys (e.g., "econ:gdp" -> "gov:stability").
/// Thread-safe for use in parallel simulation systems.
pub struct InfluenceRegistry {
    // Map from (source, target) -> weight
    edges: RwLock<HashMap<(String, String), f64>>,
}

impl InfluenceRegistry {
    pub fn new() -> Self {
        Self {
            edges: RwLock::new(HashMap::new()),
        }
    }

    /// Set the influence weight from source to target.
    pub fn set_influence(&self, source: &str, target: &str, weight: f64) {
        let mut edges = self.edges.write().unwrap();
        edges.insert((source.to_string(), target.to_string()), weight);
    }

    /// Get the influence weight from source to target. Returns 0.0 if not set.
    pub fn get_influence(&self, source: &str, target: &str) -> f64 {
        let edges = self.edges.read().unwrap();
        *edges.get(&(source.to_string(), target.to_string())).unwrap_or(&0.0)
    }
}

unsafe impl Send for InfluenceRegistry {}
unsafe impl Sync for InfluenceRegistry {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_influence_registry() {
        let reg = InfluenceRegistry::new();
        reg.set_influence("a", "b", 0.7);
        assert_eq!(reg.get_influence("a", "b"), 0.7);
        assert_eq!(reg.get_influence("b", "a"), 0.0);
    }
}
