// plugins/demog/src/plugin.rs
// use crate::state::DemogState; // removed, domain-agnostic
use crate::system::demog_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin};
// use std::any::Any; // removed, domain-agnostic

pub struct DemogPlugin;

impl SimulationPlugin for DemogPlugin {
    fn name(&self) -> &'static str {
        "demog"
    }

    fn execute(&self, _snapshot: &ReadSnapshot, _blackboard: &sdk::Blackboard) {
        // TODO: Port domain logic to open-blackboard pattern
        // This is a stub for the domain-agnostic engine
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(demog_system_system());
    }
}
