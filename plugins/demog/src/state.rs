#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct DemogState {
    pub population: u64,
    pub birth_rate: f64,
}

impl DemogState {
    /// Print field-level diff between self and another `DemogState`. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let population_changed = (self.population != other.population) && {
            println!("  population: {} → {}", self.population, other.population);
            true
        };
        let birth_rate_changed = (self.birth_rate - other.birth_rate).abs() > f64::EPSILON && {
            println!(
                "  birth_rate: {:.4} → {:.4}",
                self.birth_rate, other.birth_rate
            );
            true
        };
        population_changed || birth_rate_changed
    }
}
