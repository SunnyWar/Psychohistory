#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
}

impl EconState {
    /// Print field-level diff between self and another `EconState`. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let gdp_changed = (self.gdp - other.gdp).abs() > f64::EPSILON && {
            println!("  gdp: {:.2} → {:.2}", self.gdp, other.gdp);
            true
        };
        let inflation_changed = (self.inflation - other.inflation).abs() > f64::EPSILON && {
            println!(
                "  inflation: {:.4} → {:.4}",
                self.inflation, other.inflation
            );
            true
        };
        gdp_changed || inflation_changed
    }
}
