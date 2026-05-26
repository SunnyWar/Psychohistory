use sdk::{ReadSnapshot, SimulationTime};
use sdk::influence::InfluenceRegistry;
use std::any::Any;

/// System for computing corruption level as a function of influences and state.
pub struct CorruptionSystem<'a> {
    pub influence: &'a InfluenceRegistry,
}

impl<'a> CorruptionSystem<'a> {
    pub fn new(influence: &'a InfluenceRegistry) -> Self {
        Self { influence }
    }

    /// Computes corruption level using the influence graph for weights.
    pub fn run(
        &self,
        snapshot: &ReadSnapshot,
        state: &dyn Any, // Should be SimulationState, but type-erased for kernel
        time: SimulationTime,
    ) -> f64 {
        let avg_integrity = snapshot.get::<f64>("gov:avg_integrity").copied().unwrap_or(0.5);
        let lobbying_pressure = snapshot.get::<f64>("gov:lobbying_pressure").copied().unwrap_or(0.0);
        let donor_pressure = snapshot.get::<f64>("gov:donor_pressure").copied().unwrap_or(0.0);
        let reelection_pressure = snapshot.get::<f64>("gov:reelection_pressure").copied().unwrap_or(0.0);
        let normalized_wealth_influence = snapshot.get::<f64>("gov:normalized_wealth_influence").copied().unwrap_or(0.0);
        let faction_formation = snapshot.get::<f64>("gov:faction_formation").copied().unwrap_or(0.0);
        let bad_law_drag = snapshot.get::<f64>("law:bad_law_drag").copied().unwrap_or(0.0);
        let base = snapshot.get::<f64>("gov:corruption_base").copied().unwrap_or(0.1);
        let random_noise = snapshot.get::<f64>("core:corruption_noise").copied().unwrap_or(0.0);

        let mut acc = base + random_noise;
        acc = (1.0 - avg_integrity).mul_add(self.influence.get_influence("gov:avg_integrity", "gov:corruption"), acc);
        acc = lobbying_pressure.mul_add(self.influence.get_influence("gov:lobbying_pressure", "gov:corruption"), acc);
        acc = donor_pressure.mul_add(self.influence.get_influence("gov:donor_pressure", "gov:corruption"), acc);
        acc = reelection_pressure.mul_add(self.influence.get_influence("gov:reelection_pressure", "gov:corruption"), acc);
        acc = normalized_wealth_influence.mul_add(self.influence.get_influence("gov:normalized_wealth_influence", "gov:corruption"), acc);
        acc = faction_formation.mul_add(self.influence.get_influence("gov:faction_formation", "gov:corruption"), acc);
        acc = bad_law_drag.mul_add(self.influence.get_influence("law:bad_law_drag", "gov:corruption"), acc);
        acc.clamp(0.0, 1.0)
    }
}
