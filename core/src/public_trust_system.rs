use crate::state::{EconState, GovState};
use legion::{system, Read, Write};
use sdk::influence::InfluenceRegistry;
use sdk::SimulationTime;

#[derive(Default, Clone, Copy, Debug)]
pub struct PublicTrustMetric(pub f64);

#[system(for_each)]
pub fn public_trust_system(
    #[resource] registry: &InfluenceRegistry,
    #[resource] time: &SimulationTime,
    gov: &GovState,
    econ: &EconState,
    trust: &mut PublicTrustMetric,
) {
    let law_quality = econ.gdp; // Example: replace with actual field
    let corruption_level = gov.corruption_level;
    let prior_trust = gov.prior_trust;
    let crisis_response = gov.crisis_response;
    let legislative_speed = gov.legislative_speed;
    let bad_law_drag = gov.bad_law_drag;
    let external_shock = econ.external_shock;
    let media_impact = gov.media_impact;
    let is_gridlocked = gov.is_gridlocked;

    let gridlock_penalty = if is_gridlocked { 0.04 } else { 0.0 };
    let mut acc = -gridlock_penalty;

    acc = prior_trust.mul_add(
        registry.get_influence("gov:prior_trust", "gov:public_trust"),
        acc,
    );
    acc = law_quality.mul_add(
        registry.get_influence("law:quality", "gov:public_trust"),
        acc,
    );
    acc = crisis_response.mul_add(
        registry.get_influence("gov:crisis_response", "gov:public_trust"),
        acc,
    );
    acc = legislative_speed.mul_add(
        registry.get_influence("gov:legislative_speed", "gov:public_trust"),
        acc,
    );
    acc = corruption_level.mul_add(
        registry.get_influence("gov:corruption_level", "gov:public_trust"),
        acc,
    );
    acc = bad_law_drag.mul_add(
        registry.get_influence("law:bad_law_drag", "gov:public_trust"),
        acc,
    );
    acc = external_shock.mul_add(
        registry.get_influence("external:shock", "gov:public_trust"),
        acc,
    );
    acc = media_impact.mul_add(
        registry.get_influence("media:impact", "gov:public_trust"),
        acc,
    );

    trust.0 = acc.clamp(0.0, 1.0);
}

// Example usage (not for production):
// let registry = InfluenceRegistry::new();
// registry.set_influence("gov:prior_trust", "gov:public_trust", 0.68);
// ...
// let system = PublicTrustSystem::new(&registry);
// let trust = system.run(snapshot, state, time);
