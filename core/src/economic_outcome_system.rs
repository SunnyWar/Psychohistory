use legion::{system, Write, Read};
use sdk::influence::InfluenceRegistry;
use sdk::SimulationTime;
use crate::state::EconState;

#[derive(Default, Clone, Copy, Debug)]
pub struct EconomicOutcomeMetric(pub f64);

#[system(for_each)]
pub fn economic_outcome_system(
    #[resource] registry: &InfluenceRegistry,
    #[resource] time: &SimulationTime,
    econ: &EconState,
    outcome: &mut EconomicOutcomeMetric,
) {
    let law_quality = econ.law_quality;
    let crisis_response = econ.crisis_response;
    let adaptability = econ.adaptability;
    let policy_stock = econ.policy_stock;
    let corruption_level = econ.corruption_level;
    let bad_law_drag = econ.bad_law_drag;
    let external_shock = econ.external_shock;
    let base = econ.outcome_base;
    let economic_shock = econ.economic_shock;

    let mut acc = base + economic_shock;
    acc = law_quality.mul_add(registry.get_influence("law:quality", "econ:outcome"), acc);
    acc = crisis_response.mul_add(registry.get_influence("gov:crisis_response", "econ:outcome"), acc);
    acc = adaptability.mul_add(registry.get_influence("gov:adaptability", "econ:outcome"), acc);
    acc = policy_stock.mul_add(registry.get_influence("core:policy_stock", "econ:outcome"), acc);
    acc = corruption_level.mul_add(registry.get_influence("gov:corruption_level", "econ:outcome"), acc);
    acc = bad_law_drag.mul_add(registry.get_influence("law:bad_law_drag", "econ:outcome"), acc);
    acc = external_shock.mul_add(registry.get_influence("core:external_shock", "econ:outcome"), acc);
    outcome.0 = acc.clamp(0.0, 1.0);
}
