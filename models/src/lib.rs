#[derive(Debug, Clone, Default)]
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
}

#[derive(Debug, Clone, Default)]
pub struct DemogState {
    pub population: u64,
    pub birth_rate: f64,
}

#[derive(Debug, Clone, Default)]
pub struct GovState {
    pub tax_rate: f64,
    pub budget: f64,
    pub stability: f64,
}
