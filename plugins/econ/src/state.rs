#[derive(Debug, Clone)]
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
    pub unemployment: f64,
}

impl Default for EconState {
    fn default() -> Self {
        Self {
            gdp: 1_000_000_000.0,
            inflation: 0.02,
            unemployment: 0.05,
        }
    }
}
