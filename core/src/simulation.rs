use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::experiment::ExperimentResult;
use crate::run_experiment;
use crate::run_result::RunResult;

use crate::legal::democracy::LawProposal;
// use crate::legal::autocracy::EconomicDecree;
/// # Governance Simulation Core
///
/// This module implements the main simulation logic for governance systems, including:
/// - Output metrics (Law Quality, Corruption Level, Public Trust, Crisis Response, Adaptability, Representation Accuracy, Legislative Speed, Economic Outcome, Composite Score)
/// - Metric formulas and update rules (see `simulate_year`)
/// - Cross-domain dependencies between metrics
/// - State variables and simulation entities
/// - Plugin hooks for extensibility
///
/// ## Metric Formulas & Update Rules
/// Each metric is computed using a weighted sum of relevant state variables, configuration parameters, and random noise (where specified). All metrics are normalized to [0, 1] using `.clamp(0.0, 1.0)`.
///
/// - **Law Quality**: Affected by lobbying, donor pressure, media impact, bias, and representative efficiency.
/// - **Corruption Level**: Driven by integrity, lobbying, donor/donor pressure, reelection, wealth influence, faction formation, bad law drag, and random noise.
/// - **Public Trust**: Decays from prior trust, increased by law quality, crisis response, legislative speed, media impact; decreased by corruption, bad law drag, gridlock, and external shocks.
/// - **Crisis Response**: Combines legislative/judicial competence, leadership, expert support, policy stock, deliberation noise, legislative efficiency, and stability.
/// - **Adaptability**: Based on competence, policy stock, polarization, leadership, challenge events, faction formation, bad law drag, and gridlock.
/// - **Representation Accuracy**: Based on average representation and donor pressure.
/// - **Legislative Speed**: Product of raw speed and legislative efficiency.
/// - **Economic Outcome**: Weighted sum of law quality, crisis response, adaptability, policy stock, corruption, bad law drag, external shock, and economic shock.
/// - **Composite Score**: Weighted average of all metrics, with corruption inverted.
///
/// ## Cross-Domain Dependencies
/// - Law Quality, Crisis Response, Adaptability, Legislative Speed, Economic Outcome, and Public Trust are interdependent.
/// - Corruption Level affects Public Trust and Economic Outcome.
/// - Media Impact affects Law Quality and Public Trust.
/// - Economic Outcome includes Law Quality, Crisis Response, Adaptability, Corruption Level, and external shocks.
/// - Composite Score aggregates all metrics, inverting Corruption.
use log::{info, warn};
use rand_distr::{Distribution, Normal};
use serde_json::Value;

pub struct SimulationState {
    pub year_outcomes: Vec<YearOutcome>,
    /// History of economic decrees (autocracy only)
    pub decrees: Vec<crate::legal::autocracy::EconomicDecree>,
    /// Elite cohort for autocracy (purges, loyalty, influence)
    pub elites: Vec<crate::legal::autocracy::Elite>,
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
    /// Laws awaiting judicial review: (`LawProposal`, `scheduled_review_year`)
    pub pending_judicial_review: Vec<(LawProposal, usize)>,
}
impl Default for SimulationState {
    fn default() -> Self {
        Self {
            year_outcomes: Vec::new(),
            decrees: Vec::new(),
            elites: Vec::new(),
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
            pending_judicial_review: Vec::new(),
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
/// Recursively traverse regions in a scenario tree and run experiments for each region.
///
/// This utility loads the governance system and simulation config for each region node,
/// runs the experiment, prints results, and recurses into subregions.
///
/// - `region_name`: Name of the current region (for reporting)
/// - `node`: JSON node for the region
/// - `years`: Number of years to simulate
/// - `runs`: Number of experiment runs
/// - `print_results`: Callback to print results (`region_name`, `result`)
pub fn simulate_region_tree<F>(
    region_name: &str,
    node: &Value,
    years: usize,
    runs: usize,
    print_results: &F,
    seeds: Option<&[u64]>,
) where
    F: Fn(&str, &ExperimentResult),
{
    info!("Simulating region: {region_name} ({runs})");
    // Try to load GovernanceSystem and SimulationConfig for this region
    // Support both top-level and 'components'-nested fields
    let (system, config) = {
        let system = node
            .get("governance_system")
            .or_else(|| {
                node.get("components")
                    .and_then(|c| c.get("governance_system"))
            })
            .and_then(|gs| serde_json::from_value(gs.clone()).ok());
        let config = node
            .get("simulation_parameters")
            .or_else(|| {
                node.get("components")
                    .and_then(|c| c.get("simulation_parameters"))
            })
            .and_then(|sp| serde_json::from_value(sp.clone()).ok());
        (system, config)
    };

    if let (Some(system), Some(config)) = (system, config) {
        let plugins: Vec<Box<dyn crate::simulation::SimulationPlugin>> = vec![];
        let mut context = SimulationContext::new(config, None);
        let result: ExperimentResult =
            run_experiment(&system, years, &mut context, &plugins, runs, seeds);
        info!("Completed region: {region_name}");
        print_results(region_name, &result);
    } else {
        warn!("Skipping region '{region_name}' due to missing or invalid system/config.");
    }

    // Recurse into sub_regions
    if let Some(sub_regions) = node.get("sub_regions").and_then(|sr| sr.as_object()) {
        for (sub_name, sub_node) in sub_regions {
            let next_name = format!("{region_name}:{sub_name}");
            simulate_region_tree(&next_name, sub_node, years, runs, print_results, seeds);
        }
    }
}

pub struct EconomicInputs {
    pub law_quality: f64,
    pub corruption_level: f64,
    pub bad_law_drag: f64,
    pub external_shock: f64,
    pub policy_stock: f64,
    pub crisis_response: f64,
    pub adaptability: f64,
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
    context: &mut SimulationContext,
    year: usize,
    plugins: &[Box<dyn SimulationPlugin>],
) -> YearOutcome {
    let (lobbying_pressure, donor_pressure, law_quality) = law_quality(state, context);

    let corruption_level =
        corruption_level(state, context, year, lobbying_pressure, donor_pressure);

    let (bad_law_drag, is_gridlocked, external_shock, public_trust) =
        public_trust(state, context, law_quality, corruption_level);

    let (avg_leadership, policy_stock, legislative_efficiency, crisis_response) =
        crisis_response(state, context, year);

    let adaptability = adaptability(
        state,
        context,
        bad_law_drag,
        is_gridlocked,
        avg_leadership,
        policy_stock,
    );

    let representation_accuracy = representation_accuracy(state, donor_pressure);

    let legislative_speed = legislative_speed(context, legislative_efficiency);

    let economic_outcome = economic_outcome(
        context,
        &EconomicInputs {
            law_quality,
            corruption_level,
            bad_law_drag,
            external_shock,
            policy_stock,
            crisis_response,
            adaptability,
        },
    );

    // Composite Score
    let weights = context.config.weights;

    let mut acc = 0.0;

    acc = law_quality.mul_add(weights[0], acc);
    acc = (1.0 - corruption_level).mul_add(weights[1], acc);
    acc = public_trust.mul_add(weights[2], acc);
    acc = crisis_response.mul_add(weights[3], acc);
    acc = adaptability.mul_add(weights[4], acc);
    acc = representation_accuracy.mul_add(weights[5], acc);
    acc = legislative_speed.mul_add(weights[6], acc);
    acc = economic_outcome.mul_add(weights[7], acc);

    let weight_total: f64 = weights.iter().sum();
    let composite_score = if weight_total <= 0.0 {
        0.0
    } else {
        acc / weight_total
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

// Economic Outcome (CurrentUsSystem)
fn economic_outcome(context: &mut SimulationContext, econ: &EconomicInputs) -> f64 {
    let economic_volatility = context.config.economic_volatility;
    let economic_shock = Normal::new(0.0, economic_volatility * 0.10)
        .unwrap()
        .sample(&mut context.rand);

    let mut acc = 0.36 + economic_shock;

    acc = econ.law_quality.mul_add(0.20, acc);
    acc = econ.crisis_response.mul_add(0.13, acc);
    acc = econ.adaptability.mul_add(0.10, acc);
    acc = econ.policy_stock.mul_add(0.10, acc);
    acc = econ.corruption_level.mul_add(-0.13, acc);
    acc = econ.bad_law_drag.mul_add(-0.07, acc);
    acc = econ.external_shock.mul_add(-0.08, acc);

    acc.clamp(0.0, 1.0)
}

// Legislative Speed (FederalSensorumSystem)
fn legislative_speed(context: &SimulationContext, legislative_efficiency: f64) -> f64 {
    let raw_speed = context.config.raw_speed;
    (raw_speed * legislative_efficiency).clamp(0.0, 1.0)
}

// Representation Accuracy (CurrentUsSystem)
const fn representation_accuracy(state: &SimulationState, donor_pressure: f64) -> f64 {
    let mut acc = 0.0;

    acc = state.avg_representation.mul_add(0.90, acc);
    acc = donor_pressure.mul_add(-0.06, acc);

    acc.clamp(0.0, 1.0)
}

// Adaptability (CurrentUsSystem)
fn adaptability(
    state: &SimulationState,
    context: &SimulationContext,
    bad_law_drag: f64,
    is_gridlocked: bool,
    avg_leadership: f64,
    policy_stock: f64,
) -> f64 {
    let avg_competence = state.avg_competence;
    let partisan_polarization = context.config.partisan_polarization;
    let challenge_happened = state.challenge_happened;
    let faction_formation = state.faction_formation;

    let challenge_bonus = if challenge_happened { 0.05 } else { 0.0 };
    let gridlock_penalty = if is_gridlocked { 0.08 } else { 0.0 };

    let mut acc = 0.0;

    acc = avg_competence.mul_add(0.24, acc);
    acc = policy_stock.mul_add(0.14, acc);
    acc = (1.0 - partisan_polarization).mul_add(0.12, acc);
    acc = avg_leadership.mul_add(0.10, acc);
    acc += challenge_bonus;
    acc = faction_formation.mul_add(-0.10, acc);
    acc = bad_law_drag.mul_add(-0.08, acc);
    acc -= gridlock_penalty;

    acc.clamp(0.0, 1.0)
}

// Crisis Response (FederalSensorumSystem)
fn crisis_response(
    state: &SimulationState,
    context: &mut SimulationContext,
    year: usize,
) -> (f64, f64, f64, f64) {
    let legislative_competence = state.legislative_competence;
    let judicial_competence = state.judicial_competence;
    let avg_leadership = state.avg_leadership;
    let expert_support_effectiveness = state.expert_support_effectiveness;
    let policy_stock = state.policy_stock;
    let deliberation_noise: f64 = Normal::new(0.0, 0.1).unwrap().sample(&mut context.rand);
    log::info!(
        "[DEBUG] Run RNG: region={}, year={}, deliberation_noise={}",
        context.config.raw_law_quality,
        year,
        deliberation_noise
    );
    let legislative_efficiency = state.legislative_efficiency;
    let stability_multiplier = state.stability_multiplier;

    let mut acc = 0.0;

    acc = legislative_competence.mul_add(0.20, acc);
    acc = judicial_competence.mul_add(0.24, acc);
    acc = avg_leadership.mul_add(0.13, acc);
    acc = expert_support_effectiveness.mul_add(0.16, acc);
    acc = policy_stock.mul_add(0.11, acc);
    acc = deliberation_noise.mul_add(0.22, acc);

    let base_crisis_capability = acc;

    let crisis_response =
        (base_crisis_capability * legislative_efficiency * stability_multiplier).clamp(0.0, 1.0);

    (
        avg_leadership,
        policy_stock,
        legislative_efficiency,
        crisis_response,
    )
}

// Public Trust (CurrentUsSystem)
fn public_trust(
    state: &SimulationState,
    context: &SimulationContext,
    law_quality: f64,
    corruption_level: f64,
) -> (f64, bool, f64, f64) {
    let prior_trust = state.prior_trust;
    let public_trust_decay_rate = context.config.public_trust_decay_rate;
    let decayed_prior_trust = prior_trust * (1.0 - public_trust_decay_rate);
    let crisis_response: f64 = 0.0;
    // TODO: compute below
    let legislative_speed: f64 = 0.0;
    // Will be computed below
    let bad_law_drag = state.bad_law_drag;
    let is_gridlocked = state.is_gridlocked;
    let external_shock = state.external_shock;
    let media_impact = state.media_impact;
    let gridlock_penalty = if is_gridlocked { 0.04 } else { 0.0 };

    let mut acc = -gridlock_penalty;

    acc = decayed_prior_trust.mul_add(0.68, acc);
    acc = law_quality.mul_add(0.14, acc);
    acc = crisis_response.mul_add(0.06, acc);
    acc = legislative_speed.mul_add(0.05, acc);
    acc = corruption_level.mul_add(-0.20, acc);
    acc = bad_law_drag.mul_add(-0.08, acc);
    acc = external_shock.mul_add(-0.03, acc);
    acc = media_impact.mul_add(0.08, acc);

    let public_trust = acc.clamp(0.0, 1.0);

    (bad_law_drag, is_gridlocked, external_shock, public_trust)
}

// Example: Corruption Level (CurrentUsSystem)
fn corruption_level(
    state: &SimulationState,
    context: &mut SimulationContext,
    year: usize,
    lobbying_pressure: f64,
    donor_pressure: f64,
) -> f64 {
    let us_corruption_base = context.config.us_corruption_base;
    let avg_integrity = state.avg_integrity;
    let reelection_pressure = state.reelection_pressure;
    let us_reelection_bonus = context.config.us_reelection_bonus;
    let normalized_wealth_influence = state.normalized_wealth_influence;
    let faction_formation = state.faction_formation;
    let bad_law_drag = state.bad_law_drag;
    let random_noise = Normal::new(0.0, 0.04).unwrap().sample(&mut context.rand);
    log::info!(
        "[DEBUG] Run RNG: region={}, year={}, corruption_noise={}",
        context.config.raw_law_quality,
        year,
        random_noise
    );

    let mut acc = us_corruption_base + random_noise;

    acc = (1.0 - avg_integrity).mul_add(0.28, acc);
    acc = lobbying_pressure.mul_add(0.24, acc);
    acc = donor_pressure.mul_add(0.20, acc);
    acc = reelection_pressure.mul_add(us_reelection_bonus * 0.22, acc);
    acc = normalized_wealth_influence.mul_add(0.14, acc);
    acc = faction_formation.mul_add(0.10, acc);
    acc = bad_law_drag.mul_add(0.05, acc);

    acc.clamp(0.0, 1.0)
}

// Example: Law Quality (CurrentUsSystem)
fn law_quality(state: &SimulationState, context: &SimulationContext) -> (f64, f64, f64) {
    let lobbying_pressure = state.lobbying_pressure;
    let donor_pressure = state.donor_pressure;
    let _media_impact = state.media_impact;
    let bias_level = context.config.bias_level;
    let raw_law_quality = context.config.raw_law_quality;
    let representative_efficiency = context.config.representative_efficiency;
    let special_interest_degradation =
        (donor_pressure.mul_add(-0.12, lobbying_pressure.mul_add(-0.14, 1.0))).clamp(0.0, 1.0);
    let bias_adjustment = -bias_level.abs() * 0.02;
    let law_quality = (raw_law_quality * representative_efficiency)
        .mul_add(special_interest_degradation, bias_adjustment)
        .clamp(0.0, 1.0);
    (lobbying_pressure, donor_pressure, law_quality)
}

pub fn run_simulation(
    system: &GovernanceSystem,
    years: usize,
    context: &mut SimulationContext,
    plugins: &[Box<dyn SimulationPlugin>],
) -> RunResult {
    let mut state = SimulationState::default();
    let mut outcomes = Vec::with_capacity(years);
    let mut system = system.clone();
    for year in 0..years {
        // Membership rotation stub (implement logic as needed)
        rotate_membership(&mut system, year);
        // Main simulation logic (restored)
        let outcome = simulate_year(&system, &mut state, context, year, plugins);
        // Optionally: call legal system model here for hooks/side effects
        // let _ = system.legal_model(&GovType::Democracy).simulate_legislative_session(&system, &mut state, year, context);
        state.year_outcomes.push(outcome.clone());
        outcomes.push(outcome);
    }
    RunResult::from_outcomes(outcomes)
}
/// Rotate membership for the governance system each year.
/// Simple implementation: moves the first member to the end (cyclic rotation) if any.
fn rotate_membership(system: &mut GovernanceSystem, _year: usize) {
    if !system.members.is_empty() {
        let first = system.members.remove(0);
        system.members.push(first);
    }
}
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{SimulationConfig, SimulationContext};
    use crate::entities::GovernanceSystem;

    #[test]
    fn test_simulate_year_metrics_deterministic() {
        let mut state = SimulationState::default();
        let mut context = SimulationContext::new(SimulationConfig::default(), None);
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        let outcome = simulate_year(&system, &mut state, &mut context, 42, &plugins);
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
        let mut context = SimulationContext::new(SimulationConfig::default(), None);
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
        let c = &mut context.config;
        c.bias_level = 1.0;
        c.public_trust_decay_rate = 0.1;
        c.lobbying_strength = 5.0;
        c.wealth_influence_multiplier = 5.0;
        c.crisis_year_probability = 1.0;
        c.new_challenge_pressure = 1.0;
        c.economic_volatility = 1.0;
        c.baseline_public_trust = 0.0;
        c.media_influence_strength = 5.0;
        c.weights = [1.0; 8];
        c.us_corruption_base = 1.0;
        c.us_reelection_bonus = 5.0;
        c.partisan_polarization = 1.0;
        c.raw_law_quality = 0.0;
        c.representative_efficiency = 0.0;
        c.raw_speed = 0.0;
        let outcome = simulate_year(&system, &mut state, &mut context, 1, &plugins);
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
        let mut context = SimulationContext::new(SimulationConfig::default(), None);
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
        context.config.us_corruption_base = 1.0;
        let outcome = simulate_year(&system, &mut state, &mut context, 2, &plugins);
        // Corruption should be high, public trust and economic outcome should be low
        assert!(outcome.corruption_level > 0.8);
        assert!(outcome.public_trust < 0.5);
        assert!(outcome.economic_outcome < 0.5);
    }

    #[test]
    fn test_policy_stock_and_adaptability() {
        let mut state = SimulationState::default();
        let mut context = SimulationContext::new(SimulationConfig::default(), None);
        let system = GovernanceSystem::default();
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        state.policy_stock = 1.0;
        state.avg_competence = 1.0;
        state.avg_leadership = 1.0;
        state.challenge_happened = true;
        let outcome = simulate_year(&system, &mut state, &mut context, 3, &plugins);
        // Adaptability should be relatively high
        assert!(outcome.adaptability > 0.5);
    }
}
