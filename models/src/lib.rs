/// Economic state engine configuration.
/// Aligned to 64 bytes to match standard CPU cache line layout,
/// preventing cross-core cache invalidation when threads process distinct domains.
#[repr(align(64))]
#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize)]
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
}

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
