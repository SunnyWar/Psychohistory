//! Example: Social Cohesion Plugin for Psychohistory Core
//! Demonstrates a domain plugin using the open-blackboard architecture

use sdk::{Blackboard, ReadSnapshot, SimulationPlugin};

/// SocialCohesionPlugin: Demonstrates a non-linear social system plugin
pub struct SocialCohesionPlugin;

impl SimulationPlugin for SocialCohesionPlugin {
    fn name(&self) -> &'static str {
        "SocialCohesionPlugin"
    }

    fn execute(&self, _snapshot: &ReadSnapshot, blackboard: &Blackboard) {
        // Pull abstract parameters from the blackboard
        let social_friction = blackboard.get("social_friction");
        let cohesion_baseline = blackboard.get("cohesion_baseline");
        // Non-linear threshold: systemic stability drops sharply if friction exceeds 0.7
        let mut systemic_stability: f64 = 1.0 / (1.0 + (10.0 * (social_friction - 0.7)).exp());
        systemic_stability *= cohesion_baseline;
        // Clamp to [0, 1]
        systemic_stability = systemic_stability.clamp(0.0, 1.0);
        // Write result back to blackboard
        blackboard.set("systemic_stability", systemic_stability);
    }

    fn register_systems(&self, _schedule: &mut sdk::legion::systems::Builder) {
        // No ECS systems for this simple demonstration
    }
}

///
/// Mathematical model:
///
/// $$
/// S = \frac{1}{1 + e^{10(f - 0.7)}} \cdot B
/// $$
///
/// Where $S$ is systemic stability, $f$ is social friction, $B$ is baseline cohesion.
///
/// [Theory: Social tipping points, see Scheffer et al. (2009)]
