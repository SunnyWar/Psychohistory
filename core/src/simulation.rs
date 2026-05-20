fn clamp01(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}
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
    // Example: Law Quality (CurrentUsSystem)
    let lobbying_pressure = state.lobbying_pressure;
    let donor_pressure = state.donor_pressure;
    let media_impact = state.media_impact;
    let bias_level = 0.0; // TODO: get from config
    let raw_law_quality = 1.0; // TODO: compute from system
    let representative_efficiency = 1.0; // TODO: compute from system
    let special_interest_degradation = clamp01(1.0 - lobbying_pressure * 0.14 - donor_pressure * 0.12 - media_impact * 0.06);
    let bias_adjustment = -bias_level.abs() * 0.02;
    let law_quality = clamp01(raw_law_quality * representative_efficiency * special_interest_degradation + bias_adjustment);

    // Example: Corruption Level (CurrentUsSystem)
    let us_corruption_base = 0.0; // TODO: get from config
    let avg_integrity = state.avg_integrity;
    let reelection_pressure = state.reelection_pressure;
    let us_reelection_bonus = 1.0; // TODO: get from config
    let normalized_wealth_influence = state.normalized_wealth_influence;
    let faction_formation = state.faction_formation;
    let bad_law_drag = state.bad_law_drag;
    let random_noise = 0.0; // TODO: random value in [0,1) * 0.04
    let corruption_level = clamp01(
        us_corruption_base
        + (1.0 - avg_integrity) * 0.28
        + lobbying_pressure * 0.24
        + donor_pressure * 0.20
        + reelection_pressure * us_reelection_bonus * 0.22
        + normalized_wealth_influence * 0.14
        + faction_formation * 0.10
        + bad_law_drag * 0.05
        + random_noise
    );

    // Public Trust (CurrentUsSystem)
    let prior_trust = state.prior_trust;
    let public_trust_decay_rate = 0.015; // TODO: get from config
    let decayed_prior_trust = prior_trust * (1.0 - public_trust_decay_rate);
    let crisis_response = 0.0; // TODO: compute below
    let legislative_speed = 0.0; // TODO: compute below
    let bad_law_drag = state.bad_law_drag;
    let is_gridlocked = state.is_gridlocked;
    let external_shock = state.external_shock;
    let media_impact = state.media_impact;
    let public_trust = clamp01(
        decayed_prior_trust * 0.68
        + law_quality * 0.14
        + crisis_response * 0.06
        + legislative_speed * 0.05
        - corruption_level * 0.20
        - bad_law_drag * 0.08
        - if is_gridlocked { 0.04 } else { 0.0 }
        - external_shock * 0.03
        + media_impact * 0.08
    );

    // Crisis Response (FederalSensorumSystem)
    let legislative_competence = state.legislative_competence;
    let judicial_competence = state.judicial_competence;
    let avg_leadership = state.avg_leadership;
    let expert_support_effectiveness = state.expert_support_effectiveness;
    let policy_stock = state.policy_stock;
    let deliberation_noise = state.deliberation_noise; // TODO: random if stochastic
    let legislative_efficiency = state.legislative_efficiency;
    let stability_multiplier = state.stability_multiplier;
    let base_crisis_capability = legislative_competence * 0.20
        + judicial_competence * 0.24
        + avg_leadership * 0.13
        + expert_support_effectiveness * 0.16
        + policy_stock * 0.11
        + deliberation_noise * 0.22;
    let crisis_response = clamp01(base_crisis_capability * legislative_efficiency * stability_multiplier);

    // Adaptability (CurrentUsSystem)
    let avg_competence = state.avg_competence;
    let partisan_polarization = 0.0; // TODO: get from config
    let challenge_happened = state.challenge_happened;
    let faction_formation = state.faction_formation;
    let adaptability = clamp01(
        avg_competence * 0.24
        + policy_stock * 0.14
        + (1.0 - partisan_polarization) * 0.12
        + avg_leadership * 0.10
        + if challenge_happened { 0.05 } else { 0.0 }
        - faction_formation * 0.10
        - bad_law_drag * 0.08
        - if is_gridlocked { 0.08 } else { 0.0 }
    );

    // Representation Accuracy (CurrentUsSystem)
    let avg_representation = state.avg_representation;
    let representation_accuracy = clamp01(avg_representation * 0.90 - donor_pressure * 0.06);

    // Legislative Speed (FederalSensorumSystem)
    let raw_speed = 1.0; // TODO: compute from system
    let legislative_speed = clamp01(raw_speed * legislative_efficiency);

    // Economic Outcome (CurrentUsSystem)
    let economic_volatility = 0.20; // TODO: get from config
    let economic_shock = 0.0; // TODO: random gaussian(0, economic_volatility * 0.10)
    let economic_outcome = clamp01(
        0.36
        + law_quality * 0.20
        + crisis_response * 0.13
        + adaptability * 0.10
        + policy_stock * 0.10
        - corruption_level * 0.13
        - bad_law_drag * 0.07
        - external_shock * 0.08
        + economic_shock
    );

    // Composite Score
    let weights = [1.0; 8]; // TODO: get from config if needed
    let weighted_numerator =
        law_quality * weights[0]
        + (1.0 - corruption_level) * weights[1]
        + public_trust * weights[2]
        + crisis_response * weights[3]
        + adaptability * weights[4]
        + representation_accuracy * weights[5]
        + legislative_speed * weights[6]
        + economic_outcome * weights[7];
    let weight_total: f64 = weights.iter().sum();
    let composite_score = if weight_total <= 0.0 { 0.0 } else { weighted_numerator / weight_total };

    let mut outcome = YearOutcome {
        law_quality,
        corruption_level,
        public_trust,
        crisis_response,
        adaptability,
        representation_accuracy,
        legislative_speed,
        economic_outcome,
        composite_score,
    };
    for plugin in plugins {
        plugin.modify_outcome(system, state, year, &mut outcome);
    }
    outcome
}

pub fn run_simulation(system: &GovernanceSystem, years: usize, plugins: &[Box<dyn SimulationPlugin>]) -> Vec<YearOutcome> {
    let mut state = SimulationState::default();
    let mut outcomes = Vec::with_capacity(years);
    let mut system = system.clone();
    for year in 0..years {
        // Membership rotation stub (implement logic as needed)
        rotate_membership(&mut system, year);
        let outcome = simulate_year(&system, &mut state, year, plugins);
        state.year_outcomes.push(outcome.clone());
        outcomes.push(outcome);
    }
    outcomes
}

fn rotate_membership(system: &mut GovernanceSystem, year: usize) {
    // TODO: Implement membership rotation logic if required by the simulation
    // For now, this is a stub.
}
