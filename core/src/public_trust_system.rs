use sdk::{ReadSnapshot, SimulationTime};
use sdk::influence::InfluenceRegistry;
use std::any::Any;

/// System for computing public trust as a function of influences and state.
pub struct PublicTrustSystem<'a> {
    pub influence: &'a InfluenceRegistry,
}

impl<'a> PublicTrustSystem<'a> {
    pub fn new(influence: &'a InfluenceRegistry) -> Self {
        Self { influence }
    }

    /// Computes public trust using the influence graph for weights.
    pub fn run(
        &self,
        snapshot: &ReadSnapshot,
        state: &dyn Any, // Should be SimulationState, but type-erased for kernel
        time: SimulationTime,
    ) -> f64 {
        // Example: get influences for law_quality, corruption_level, etc.
        let law_quality = snapshot.get::<f64>("law:quality").copied().unwrap_or(0.0);
        let corruption_level = snapshot.get::<f64>("gov:corruption_level").copied().unwrap_or(0.0);
        let prior_trust = snapshot.get::<f64>("gov:prior_trust").copied().unwrap_or(0.5);
        let crisis_response = snapshot.get::<f64>("gov:crisis_response").copied().unwrap_or(0.0);
        let legislative_speed = snapshot.get::<f64>("gov:legislative_speed").copied().unwrap_or(0.0);
        let bad_law_drag = snapshot.get::<f64>("law:bad_law_drag").copied().unwrap_or(0.0);
        let external_shock = snapshot.get::<f64>("external:shock").copied().unwrap_or(0.0);
        let media_impact = snapshot.get::<f64>("media:impact").copied().unwrap_or(0.0);
        let is_gridlocked = snapshot.get::<bool>("gov:is_gridlocked").copied().unwrap_or(false);

        let gridlock_penalty = if is_gridlocked { 0.04 } else { 0.0 };
        let mut acc = -gridlock_penalty;

        acc = prior_trust.mul_add(self.influence.get_influence("gov:prior_trust", "gov:public_trust"), acc);
        acc = law_quality.mul_add(self.influence.get_influence("law:quality", "gov:public_trust"), acc);
        acc = crisis_response.mul_add(self.influence.get_influence("gov:crisis_response", "gov:public_trust"), acc);
        acc = legislative_speed.mul_add(self.influence.get_influence("gov:legislative_speed", "gov:public_trust"), acc);
        acc = corruption_level.mul_add(self.influence.get_influence("gov:corruption_level", "gov:public_trust"), acc);
        acc = bad_law_drag.mul_add(self.influence.get_influence("law:bad_law_drag", "gov:public_trust"), acc);
        acc = external_shock.mul_add(self.influence.get_influence("external:shock", "gov:public_trust"), acc);
        acc = media_impact.mul_add(self.influence.get_influence("media:impact", "gov:public_trust"), acc);

        acc.clamp(0.0, 1.0)
    }
}

// Example usage (not for production):
// let registry = InfluenceRegistry::new();
// registry.set_influence("gov:prior_trust", "gov:public_trust", 0.68);
// ...
// let system = PublicTrustSystem::new(&registry);
// let trust = system.run(snapshot, state, time);
