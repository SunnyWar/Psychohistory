use crate::entities::{GovernanceSystem, YearOutcome};

pub struct SimulationState {
    pub year_outcomes: Vec<YearOutcome>,
    pub prior_trust: f64,
    pub policy_stock: f64,
    pub avg_competence: f64,
    pub avg_integrity: f64,
    pub avg_leadership: f64,
    pub avg_representation: f64,
    pub lobbying_pressure: f64,
    pub donor_pressure: f64,
    pub media_impact: f64,
    pub reelection_pressure: f64,
    pub normalized_wealth_influence: f64,
    pub faction_formation: f64,
    pub bad_law_drag: f64,
    pub is_gridlocked: bool,
    pub external_shock: f64,
    pub challenge_happened: bool,
    pub legislative_efficiency: f64,
    pub deliberation_noise: f64,
    pub deliberation_bonus: f64,
    pub evidence_board_effect: f64,
    pub cohort_quality_shock: f64,
    pub stability_multiplier: f64,
    pub legislative_competence: f64,
    pub judicial_competence: f64,
    pub expert_support_effectiveness: f64,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            year_outcomes: Vec::new(),
            prior_trust: 0.5,
            policy_stock: 0.0,
            avg_competence: 0.0,
            avg_integrity: 0.0,
            avg_leadership: 0.0,
            avg_representation: 0.0,
            lobbying_pressure: 0.0,
            donor_pressure: 0.0,
            media_impact: 0.0,
            reelection_pressure: 0.0,
            normalized_wealth_influence: 0.0,
            faction_formation: 0.0,
            bad_law_drag: 0.0,
            is_gridlocked: false,
            external_shock: 0.0,
            challenge_happened: false,
            legislative_efficiency: 0.0,
            deliberation_noise: 0.0,
            deliberation_bonus: 0.0,
            evidence_board_effect: 0.0,
            cohort_quality_shock: 0.0,
            stability_multiplier: 1.0,
            legislative_competence: 0.0,
            judicial_competence: 0.0,
            expert_support_effectiveness: 0.0,
        }
    }
}


pub trait SimulationPlugin {
    fn modify_outcome(&self, system: &GovernanceSystem, state: &SimulationState, year: usize, outcome: &mut YearOutcome);
}

pub fn simulate_year(
    system: &GovernanceSystem,
    state: &mut SimulationState,
    year: usize,
    plugins: &[Box<dyn SimulationPlugin>],
) -> YearOutcome {
    // TODO: Implement metric formulas and update rules
    let mut outcome = YearOutcome::default();
    for plugin in plugins {
        plugin.modify_outcome(system, state, year, &mut outcome);
    }
    outcome
}

pub fn run_simulation(system: &GovernanceSystem, years: usize, plugins: &[Box<dyn SimulationPlugin>]) -> Vec<YearOutcome> {
    let mut state = SimulationState::default();
    let mut outcomes = Vec::with_capacity(years);
    for year in 0..years {
        let outcome = simulate_year(system, &mut state, year, plugins);
        state.year_outcomes.push(outcome.clone());
        outcomes.push(outcome);
    }
    outcomes
}
