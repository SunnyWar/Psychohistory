use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Legislator {
    pub id: String,
    pub chamber: ChamberType,
    pub competence: f64,
    pub integrity: f64,
    pub ideology: f64,
    pub wealth: f64,
    pub is_experienced: bool,
    pub representativeness: f64,
    pub leadership_quality: f64,
    pub faction_affinity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChamberType {
    House,
    Senate,
    LegislativeAssembly,
    JudicialTribunal,
}

impl Default for ChamberType {
    fn default() -> Self {
        ChamberType::House
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernanceSystem {
    pub members: Vec<Legislator>,
    pub system_kind: SystemKind,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemKind {
    CurrentUsSystem,
    FederalSensorumSystem,
}

impl Default for SystemKind {
    fn default() -> Self {
        SystemKind::CurrentUsSystem
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct YearOutcome {
    pub law_quality: f64,
    pub corruption_level: f64,
    pub public_trust: f64,
    pub crisis_response: f64,
    pub adaptability: f64,
    pub representation_accuracy: f64,
    pub legislative_speed: f64,
    pub economic_outcome: f64,
    pub composite_score: f64,
}
