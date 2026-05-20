/// Economic system type (market, planned, mixed, etc.)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EconSystemType {
    Market,
    Planned,
    Mixed,
}

/// Government type (democracy, autocracy, etc.)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GovType {
    Democracy,
    Autocracy,
    Monarchy,
    Other(String),
}

/// Economic state engine configuration.
/// Aligned to 64 bytes to match standard CPU cache line layout,
/// preventing cross-core cache invalidation when threads process distinct domains.
#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
}

/// Demographic state component tracking macro population metrics.
#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct DemogState {
    pub population: u64,
    pub birth_rate: f64,
}

/// Governance state metrics managing fiscal rules and societal stability.
#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct GovState {
    pub tax_rate: f64,
    pub budget: f64,
    pub stability: f64,
}

impl EconState {
    /// Print field-level diff between self and another EconState. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let mut changed = false;
        if (self.gdp - other.gdp).abs() > f64::EPSILON {
            println!("  gdp: {:.2} → {:.2}", self.gdp, other.gdp);
            changed = true;
        }
        if (self.inflation - other.inflation).abs() > f64::EPSILON {
            println!(
                "  inflation: {:.4} → {:.4}",
                self.inflation, other.inflation
            );
            changed = true;
        }
        changed
    }
}

impl DemogState {
    /// Print field-level diff between self and another DemogState. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let mut changed = false;
        if self.population != other.population {
            println!("  population: {} → {}", self.population, other.population);
            changed = true;
        }
        if (self.birth_rate - other.birth_rate).abs() > f64::EPSILON {
            println!(
                "  birth_rate: {:.4} → {:.4}",
                self.birth_rate, other.birth_rate
            );
            changed = true;
        }
        changed
    }
}

impl GovState {
    /// Print field-level diff between self and another GovState. Returns true if any field changed.
    pub fn print_diff(&self, other: &Self) -> bool {
        let mut changed = false;
        if (self.tax_rate - other.tax_rate).abs() > f64::EPSILON {
            println!("  tax_rate: {:.4} → {:.4}", self.tax_rate, other.tax_rate);
            changed = true;
        }
        if (self.budget - other.budget).abs() > f64::EPSILON {
            println!("  budget: {:.2} → {:.2}", self.budget, other.budget);
            changed = true;
        }
        if (self.stability - other.stability).abs() > f64::EPSILON {
            println!(
                "  stability: {:.4} → {:.4}",
                self.stability, other.stability
            );
            changed = true;
        }
        changed
    }
}
