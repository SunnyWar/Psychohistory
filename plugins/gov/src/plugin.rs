// plugins/gov/src/plugin.rs
// use crate::state::GovState; // removed, domain-agnostic
use crate::system::gov_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin};
// use std::any::Any; // removed, domain-agnostic

pub struct GovPlugin;

impl SimulationPlugin for GovPlugin {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn execute(&self, _snapshot: &ReadSnapshot, _blackboard: &sdk::Blackboard) {
        // TODO: Port domain logic to open-blackboard pattern
        // This is a stub for the domain-agnostic engine
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(gov_system_system());
    }
}
