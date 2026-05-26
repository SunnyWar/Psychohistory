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
