use sdk::{ReadSnapshot, SimulationTime};
use sdk::influence::InfluenceRegistry;
use std::any::Any;

/// System for computing economic outcome as a function of influences and state.
pub struct EconomicOutcomeSystem<'a> {
    pub influence: &'a InfluenceRegistry,
}

impl<'a> EconomicOutcomeSystem<'a> {
    pub fn new(influence: &'a InfluenceRegistry) -> Self {
        Self { influence }
    }

    /// Computes economic outcome using the influence graph for weights.
    pub fn run(
        &self,
        snapshot: &ReadSnapshot,
        state: &dyn Any, // Should be SimulationState, but type-erased for kernel
        time: SimulationTime,
    ) -> f64 {
        let law_quality = snapshot.get::<f64>("law:quality").copied().unwrap_or(0.0);
        let crisis_response = snapshot.get::<f64>("gov:crisis_response").copied().unwrap_or(0.0);
        let adaptability = snapshot.get::<f64>("gov:adaptability").copied().unwrap_or(0.0);
        let policy_stock = snapshot.get::<f64>("core:policy_stock").copied().unwrap_or(0.0);
        let corruption_level = snapshot.get::<f64>("gov:corruption_level").copied().unwrap_or(0.0);
        let bad_law_drag = snapshot.get::<f64>("law:bad_law_drag").copied().unwrap_or(0.0);
        let external_shock = snapshot.get::<f64>("core:external_shock").copied().unwrap_or(0.0);
        let base = snapshot.get::<f64>("econ:outcome_base").copied().unwrap_or(0.36);
        let economic_shock = snapshot.get::<f64>("econ:economic_shock").copied().unwrap_or(0.0);

        let mut acc = base + economic_shock;
        acc = law_quality.mul_add(self.influence.get_influence("law:quality", "econ:outcome"), acc);
        acc = crisis_response.mul_add(self.influence.get_influence("gov:crisis_response", "econ:outcome"), acc);
        acc = adaptability.mul_add(self.influence.get_influence("gov:adaptability", "econ:outcome"), acc);
        acc = policy_stock.mul_add(self.influence.get_influence("core:policy_stock", "econ:outcome"), acc);
        acc = corruption_level.mul_add(self.influence.get_influence("gov:corruption_level", "econ:outcome"), acc);
        acc = bad_law_drag.mul_add(self.influence.get_influence("law:bad_law_drag", "econ:outcome"), acc);
        acc = external_shock.mul_add(self.influence.get_influence("core:external_shock", "econ:outcome"), acc);
        acc.clamp(0.0, 1.0)
    }
}
