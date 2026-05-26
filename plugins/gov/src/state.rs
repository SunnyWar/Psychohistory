#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct GovState {
    pub tax_rate: f64,
    pub budget: f64,
    pub stability: f64,
}

impl GovState {
    /// Print field-level diff between self and another `GovState`. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let tax_changed = (self.tax_rate - other.tax_rate).abs() > f64::EPSILON && {
            println!("  tax_rate: {:.4} → {:.4}", self.tax_rate, other.tax_rate);
            true
        };
        let budget_changed = (self.budget - other.budget).abs() > f64::EPSILON && {
            println!("  budget: {:.2} → {:.2}", self.budget, other.budget);
            true
        };
        let stability_changed = (self.stability - other.stability).abs() > f64::EPSILON && {
            println!(
                "  stability: {:.4} → {:.4}",
                self.stability, other.stability
            );
            true
        };
        tax_changed || budget_changed || stability_changed
    }
}
