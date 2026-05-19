use crate::plugin::EconPlugin;
use core::{system::System, time::SimulationTime};
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;

pub struct EconSystem;

impl System for EconSystem {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn run(&mut self, _state: &mut core::state::SimulationState, _time: SimulationTime) {
        // Legacy single-threaded fallback loop - intentionally left blank
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        _time: SimulationTime,
    ) {
        // Dispatch directly into the parallel simulation execution block
        EconPlugin.step(snapshot, bucket);
    }
}
// plugins/econ/src/system.rs
use models::{DemogState, EconState};

pub fn run_econ_system(world: &ReadSnapshot, my_state: &mut Box<dyn std::any::Any + Send + Sync>) {
    // 1. Downcast your own mutable slice directly from the passed-in block
    let econ = my_state
        .downcast_mut::<EconState>()
        .expect("Failed to downcast to EconState");

    // 2. Read safely from other states using the snapshot
    if let Some(demog) = world.get::<DemogState>("demog")
        && demog.population > 10_000_000
    {
        econ.gdp += 500_000.0;
    }
}
