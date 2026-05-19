use core::{system::System, time::SimulationTime};
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;
use crate::plugin::DemogPlugin;

pub struct DemogSystem;

impl System for DemogSystem {
    fn name(&self) -> &'static str {
        "demog"
    }

    fn run(&mut self, _state: &mut core::state::SimulationState, _time: SimulationTime) {
        // Obsolete legacy sequential hook—can remain empty or log a warning
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        _time: SimulationTime,
    ) {
        // Forward execution to the thread-safe plugin step function
        DemogPlugin.step(snapshot, bucket);
    }
}
// plugins/demog/src/system.rs
