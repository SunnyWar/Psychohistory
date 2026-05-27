// plugins/econ/src/plugin.rs
// use crate::state::EconState; // removed, domain-agnostic
use crate::system::econ_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin};
// use std::any::Any; // removed, domain-agnostic

pub struct EconPlugin;

impl SimulationPlugin for EconPlugin {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn execute(&self, _snapshot: &ReadSnapshot, _blackboard: &sdk::Blackboard) {
        // TODO: Port domain logic to open-blackboard pattern
        // This is a stub for the domain-agnostic engine
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(econ_system_system());
    }
}
