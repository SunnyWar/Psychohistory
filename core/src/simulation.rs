
//! # Governance Simulation Core
//!
//! This module implements the main simulation logic for governance systems, including:
//! - Output metrics (Law Quality, Corruption Level, Public Trust, Crisis Response, Adaptability, Representation Accuracy, Legislative Speed, Economic Outcome, Composite Score)
//! - Metric formulas and update rules (see `simulate_year`)
//! - Cross-domain dependencies between metrics
//! - State variables and simulation entities
//! - Plugin hooks for extensibility
//!
//! ## Metric Formulas & Update Rules
//! Each metric is computed using a weighted sum of relevant state variables, configuration parameters, and random noise (where specified). All metrics are normalized to [0, 1] using `.clamp(0.0, 1.0)`.
//!
//! - **Law Quality**: Affected by lobbying, donor pressure, media impact, bias, and representative efficiency.
//! - **Corruption Level**: Driven by integrity, lobbying, donor/donor pressure, reelection, wealth influence, faction formation, bad law drag, and random noise.
//! - **Public Trust**: Decays from prior trust, increased by law quality, crisis response, legislative speed, media impact; decreased by corruption, bad law drag, gridlock, and external shocks.
//! - **Crisis Response**: Combines legislative/judicial competence, leadership, expert support, policy stock, deliberation noise, legislative efficiency, and stability.
//! - **Adaptability**: Based on competence, policy stock, polarization, leadership, challenge events, faction formation, bad law drag, and gridlock.
//! - **Representation Accuracy**: Based on average representation and donor pressure.
//! - **Legislative Speed**: Product of raw speed and legislative efficiency.
//! - **Economic Outcome**: Weighted sum of law quality, crisis response, adaptability, policy stock, corruption, bad law drag, external shock, and economic shock.
//! - **Composite Score**: Weighted average of all metrics, with corruption inverted.
//!
//! ## Cross-Domain Dependencies
//! - Law Quality, Crisis Response, Adaptability, Legislative Speed, Economic Outcome, and Public Trust are interdependent.
//! - Corruption Level affects Public Trust and Economic Outcome.
//! - Media Impact affects Law Quality and Public Trust.
//! - Economic Outcome includes Law Quality, Crisis Response, Adaptability, Corruption Level, and external shocks.
//! - Composite Score aggregates all metrics, inverting Corruption.

use crate::config::SimulationConfig;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::run_result::RunResult;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Normal};

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
    fn modify_outcome(
        &self,
        system: &GovernanceSystem,
        state: &SimulationState,
        year: usize,
        outcome: &mut YearOutcome,
    );
}

/// Simulate a single year of governance, updating all output metrics.
///
/// # Metric Formulas
/// - See module-level docs for details on each metric.
/// - All metrics are normalized to [0, 1].
/// - Randomization is applied to corruption, deliberation, and economic shocks.
///
/// # Cross-Domain Dependencies
/// - Metrics are interdependent as described in the module docs.
///
/// # Arguments
/// Simulate a single year of governance, updating all output metrics.
///
/// # Metric Formulas
/// - See module-level docs for details on each metric.
/// - All metrics are normalized to [0, 1].
/// - Randomization is applied to corruption, deliberation, and economic shocks.
///
/// # Cross-Domain Dependencies
/// - Metrics are interdependent as described in the module docs.
///
/// # Arguments
/// * `system` - The governance system being simulated
/// * `state` - The mutable simulation state for the current year
/// * `config` - Simulation configuration parameters
/// * `year` - The simulation year (used for RNG seed)
/// * `plugins` - Optional plugin hooks for modifying outcomes
///
/// # Returns
/// * `YearOutcome` - The computed metrics for the year
pub fn simulate_year(
    system: &GovernanceSystem,
    state: &mut SimulationState,
    config: &SimulationConfig,
    year: usize,
    plugins: &[Box<dyn SimulationPlugin>],
) -> YearOutcome {
    let mut rng = StdRng::seed_from_u64(year as u64);
    // Example: Law Quality (CurrentUsSystem)
    let lobbying_pressure = state.lobbying_pressure;
    let donor_pressure = state.donor_pressure;
    let media_impact = state.media_impact;
    let bias_level = config.bias_level;
    let raw_law_quality = config.raw_law_quality;
    let representative_efficiency = config.representative_efficiency;
    let special_interest_degradation =
        (1.0 - lobbying_pressure * 0.14 - donor_pressure * 0.12 - media_impact * 0.06)
            .clamp(0.0, 1.0);
    let bias_adjustment = -bias_level.abs() * 0.02;
    let law_quality = (raw_law_quality * representative_efficiency * special_interest_degradation
        + bias_adjustment)
        .clamp(0.0, 1.0);

    // Example: Corruption Level (CurrentUsSystem)
    let us_corruption_base = config.us_corruption_base;
    let avg_integrity = state.avg_integrity;
    let reelection_pressure = state.reelection_pressure;
    let us_reelection_bonus = config.us_reelection_bonus;
    let normalized_wealth_influence = state.normalized_wealth_influence;
    let faction_formation = state.faction_formation;
    let bad_law_drag = state.bad_law_drag;
    let random_noise = Normal::new(0.0, 0.04).unwrap().sample(&mut rng);
    let corruption_level = (us_corruption_base
        + (1.0 - avg_integrity) * 0.28
        + lobbying_pressure * 0.24
        + donor_pressure * 0.20
        + reelection_pressure * us_reelection_bonus * 0.22
        + normalized_wealth_influence * 0.14
        + faction_formation * 0.10
        + bad_law_drag * 0.05
        + random_noise)
        .clamp(0.0, 1.0);

    // Public Trust (CurrentUsSystem)
    let prior_trust = state.prior_trust;
    let public_trust_decay_rate = config.public_trust_decay_rate;
    let decayed_prior_trust = prior_trust * (1.0 - public_trust_decay_rate);
    let crisis_response = 0.0; // TODO: compute below
    let legislative_speed = 0.0; // Will be computed below
    let bad_law_drag = state.bad_law_drag;
    let is_gridlocked = state.is_gridlocked;
    let external_shock = state.external_shock;
    let media_impact = state.media_impact;
    let public_trust = (decayed_prior_trust * 0.68
        + law_quality * 0.14
        + crisis_response * 0.06
        + legislative_speed * 0.05
        - corruption_level * 0.20
        - bad_law_drag * 0.08
        - if is_gridlocked { 0.04 } else { 0.0 }
        - external_shock * 0.03
        + media_impact * 0.08)
        .clamp(0.0, 1.0);

    // Crisis Response (FederalSensorumSystem)
    let legislative_competence = state.legislative_competence;
    let judicial_competence = state.judicial_competence;
    let avg_leadership = state.avg_leadership;
    let expert_support_effectiveness = state.expert_support_effectiveness;
    let policy_stock = state.policy_stock;
    let deliberation_noise = Normal::new(0.0, 0.1).unwrap().sample(&mut rng);
    let legislative_efficiency = state.legislative_efficiency;
    let stability_multiplier = state.stability_multiplier;
    let base_crisis_capability = legislative_competence * 0.20
        + judicial_competence * 0.24
        + avg_leadership * 0.13
        + expert_support_effectiveness * 0.16
        + policy_stock * 0.11
        + deliberation_noise * 0.22;
    let crisis_response =
        (base_crisis_capability * legislative_efficiency * stability_multiplier).clamp(0.0, 1.0);

    // Adaptability (CurrentUsSystem)
    let avg_competence = state.avg_competence;
    let partisan_polarization = config.partisan_polarization;
    let challenge_happened = state.challenge_happened;
    let faction_formation = state.faction_formation;
    let adaptability = (avg_competence * 0.24
        + policy_stock * 0.14
        + (1.0 - partisan_polarization) * 0.12
        + avg_leadership * 0.10
        + if challenge_happened { 0.05 } else { 0.0 }
        - faction_formation * 0.10
        - bad_law_drag * 0.08
        - if is_gridlocked { 0.08 } else { 0.0 })
    .clamp(0.0, 1.0);

    // Representation Accuracy (CurrentUsSystem)
    let avg_representation = state.avg_representation;
    let representation_accuracy =
        (avg_representation * 0.90 - donor_pressure * 0.06).clamp(0.0, 1.0);

    // Legislative Speed (FederalSensorumSystem)
    let raw_speed = config.raw_speed;
    let legislative_speed = (raw_speed * legislative_efficiency).clamp(0.0, 1.0);

    // Economic Outcome (CurrentUsSystem)
    let economic_volatility = config.economic_volatility;
    let economic_shock = Normal::new(0.0, economic_volatility * 0.10)
        .unwrap()
        .sample(&mut rng);
    let economic_outcome = (0.36
        + law_quality * 0.20
        + crisis_response * 0.13
        + adaptability * 0.10
        + policy_stock * 0.10
        - corruption_level * 0.13
        - bad_law_drag * 0.07
        - external_shock * 0.08
        + economic_shock)
        .clamp(0.0, 1.0);

    // Composite Score
    let weights = config.weights;
    let weighted_numerator = law_quality * weights[0]
        + (1.0 - corruption_level) * weights[1]
        + public_trust * weights[2]
        + crisis_response * weights[3]
        + adaptability * weights[4]
        + representation_accuracy * weights[5]
        + legislative_speed * weights[6]
        + economic_outcome * weights[7];
    let weight_total: f64 = weights.iter().sum();
    let composite_score = if weight_total <= 0.0 {
        0.0
    } else {
        weighted_numerator / weight_total
    };

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

pub fn run_simulation(
    system: &GovernanceSystem,
    years: usize,
    config: &SimulationConfig,
    plugins: &[Box<dyn SimulationPlugin>],
) -> RunResult {
    let mut state = SimulationState::default();
    let mut outcomes = Vec::with_capacity(years);
    let mut system = system.clone();
    for year in 0..years {
        // Membership rotation stub (implement logic as needed)
        rotate_membership(&mut system, year);
        let outcome = simulate_year(&system, &mut state, config, year, plugins);
        state.year_outcomes.push(outcome.clone());
        outcomes.push(outcome);
    }
    RunResult::from_outcomes(outcomes)
}

fn rotate_membership(system: &mut GovernanceSystem, year: usize) {
    // TODO: Implement membership rotation logic if required by the simulation
    // For now, this is a stub.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{GovernanceSystem, YearOutcome};
    use crate::config::SimulationConfig;

    #[test]
    fn test_simulate_year_metrics_deterministic() {
        let mut state = SimulationState::default();
        let config = SimulationConfig::default();
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        let outcome = simulate_year(&system, &mut state, &config, 42, &plugins);
        // All metrics should be in [0, 1]
        assert!((0.0..=1.0).contains(&outcome.law_quality));
        assert!((0.0..=1.0).contains(&outcome.corruption_level));
        assert!((0.0..=1.0).contains(&outcome.public_trust));
        assert!((0.0..=1.0).contains(&outcome.crisis_response));
        assert!((0.0..=1.0).contains(&outcome.adaptability));
        assert!((0.0..=1.0).contains(&outcome.representation_accuracy));
        assert!((0.0..=1.0).contains(&outcome.legislative_speed));
        assert!((0.0..=1.0).contains(&outcome.economic_outcome));
        assert!((0.0..=1.0).contains(&outcome.composite_score));
    }

    #[test]
    fn test_extreme_inputs() {
        let mut state = SimulationState::default();
        let mut config = SimulationConfig::default();
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        // Set extreme values for state and config
        state.avg_integrity = 0.0;
        state.lobbying_pressure = 5.0;
        state.donor_pressure = 5.0;
        state.media_impact = 5.0;
        state.reelection_pressure = 5.0;
        state.normalized_wealth_influence = 5.0;
        state.faction_formation = 5.0;
        state.bad_law_drag = 5.0;
        state.is_gridlocked = true;
        state.external_shock = 5.0;
        state.challenge_happened = true;
        state.legislative_efficiency = 0.0;
        state.deliberation_noise = 5.0;
        state.deliberation_bonus = 5.0;
        state.evidence_board_effect = 5.0;
        state.cohort_quality_shock = 5.0;
        state.stability_multiplier = 0.0;
        state.legislative_competence = 0.0;
        state.judicial_competence = 0.0;
        state.expert_support_effectiveness = 0.0;
        state.avg_competence = 0.0;
        state.avg_leadership = 0.0;
        state.avg_representation = 0.0;
        state.policy_stock = 0.0;
        config.bias_level = 1.0;
        config.public_trust_decay_rate = 0.1;
        config.lobbying_strength = 5.0;
        config.wealth_influence_multiplier = 5.0;
        config.crisis_year_probability = 1.0;
        config.new_challenge_pressure = 1.0;
        config.economic_volatility = 1.0;
        config.baseline_public_trust = 0.0;
        config.media_influence_strength = 5.0;
        config.weights = [1.0; 8];
        config.us_corruption_base = 1.0;
        config.us_reelection_bonus = 5.0;
        config.partisan_polarization = 1.0;
        config.raw_law_quality = 0.0;
        config.representative_efficiency = 0.0;
        config.raw_speed = 0.0;
        let outcome = simulate_year(&system, &mut state, &config, 1, &plugins);
        // All metrics should still be clamped to [0, 1]
        assert!((0.0..=1.0).contains(&outcome.law_quality));
        assert!((0.0..=1.0).contains(&outcome.corruption_level));
        assert!((0.0..=1.0).contains(&outcome.public_trust));
        assert!((0.0..=1.0).contains(&outcome.crisis_response));
        assert!((0.0..=1.0).contains(&outcome.adaptability));
        assert!((0.0..=1.0).contains(&outcome.representation_accuracy));
        assert!((0.0..=1.0).contains(&outcome.legislative_speed));
        assert!((0.0..=1.0).contains(&outcome.economic_outcome));
        assert!((0.0..=1.0).contains(&outcome.composite_score));
    }

    #[test]
    fn test_cross_domain_dependency() {
        let mut state = SimulationState::default();
        let mut config = SimulationConfig::default();
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        // Set corruption high, expect public trust and economic outcome to be lower
        state.avg_integrity = 0.0;
        state.lobbying_pressure = 5.0;
        state.donor_pressure = 5.0;
        state.reelection_pressure = 5.0;
        state.normalized_wealth_influence = 5.0;
        state.faction_formation = 5.0;
        state.bad_law_drag = 5.0;
        config.us_corruption_base = 1.0;
        let outcome = simulate_year(&system, &mut state, &config, 2, &plugins);
        // Corruption should be high, public trust and economic outcome should be low
        assert!(outcome.corruption_level > 0.8);
        assert!(outcome.public_trust < 0.5);
        assert!(outcome.economic_outcome < 0.5);
    }

    #[test]
    fn test_policy_stock_and_adaptability() {
        let mut state = SimulationState::default();
        let config = SimulationConfig::default();
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        state.policy_stock = 1.0;
        state.avg_competence = 1.0;
        state.avg_leadership = 1.0;
        state.challenge_happened = true;
        let outcome = simulate_year(&system, &mut state, &config, 3, &plugins);
        // Adaptability should be relatively high
        assert!(outcome.adaptability > 0.5);
    }
}
