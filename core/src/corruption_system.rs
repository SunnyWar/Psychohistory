use legion::{system, Write, Read};
use sdk::influence::InfluenceRegistry;
use sdk::SimulationTime;
use crate::state::GovState;

#[derive(Default, Clone, Copy, Debug)]
pub struct CorruptionMetric(pub f64);

#[system(for_each)]
pub fn corruption_system(
    #[resource] registry: &InfluenceRegistry,
    #[resource] time: &SimulationTime,
    gov: &GovState,
    corruption: &mut CorruptionMetric,
) {
    let avg_integrity = gov.avg_integrity;
    let lobbying_pressure = gov.lobbying_pressure;
    let donor_pressure = gov.donor_pressure;
    let reelection_pressure = gov.reelection_pressure;
    let normalized_wealth_influence = gov.normalized_wealth_influence;
    let faction_formation = gov.faction_formation;
    let bad_law_drag = gov.bad_law_drag;
    let base = gov.corruption_base;
    let random_noise = gov.corruption_noise;

    let mut acc = base + random_noise;
    acc = (1.0 - avg_integrity).mul_add(registry.get_influence("gov:avg_integrity", "gov:corruption"), acc);
    acc = lobbying_pressure.mul_add(registry.get_influence("gov:lobbying_pressure", "gov:corruption"), acc);
    acc = donor_pressure.mul_add(registry.get_influence("gov:donor_pressure", "gov:corruption"), acc);
    acc = reelection_pressure.mul_add(registry.get_influence("gov:reelection_pressure", "gov:corruption"), acc);
    acc = normalized_wealth_influence.mul_add(registry.get_influence("gov:normalized_wealth_influence", "gov:corruption"), acc);
    acc = faction_formation.mul_add(registry.get_influence("gov:faction_formation", "gov:corruption"), acc);
    acc = bad_law_drag.mul_add(registry.get_influence("law:bad_law_drag", "gov:corruption"), acc);
    corruption.0 = acc.clamp(0.0, 1.0);
}
