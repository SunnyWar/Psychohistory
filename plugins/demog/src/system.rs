use crate::plugin::DemogPlugin;
use core::{SimulationState, System};
use sdk::SimulationTime;
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;

pub struct DemogSystem;

impl System for DemogSystem {
    fn name(&self) -> &'static str {
        "demog"
    }

    fn run(&mut self, _state: &mut SimulationState, _time: SimulationTime) {
        // Obsolete legacy sequential hook—can remain empty or log a warning
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
    ) {
        // Forward execution to the thread-safe plugin step function
        DemogPlugin.step(snapshot, bucket, time);
    }
}
// plugins/demog/src/system.rs
