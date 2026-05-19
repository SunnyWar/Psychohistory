use core::{system::System, time::SimulationTime};
use sdk::ReadSnapshot;
use std::any::Any;

pub struct EconSystem;

impl System for EconSystem {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn run(&mut self, _state: &mut core::state::SimulationState, _time: SimulationTime) {
        // Not implemented
    }

    fn run_system(
        &self,
        _snapshot: &ReadSnapshot,
        _bucket: &mut Box<dyn Any + Send + Sync>,
        _time: SimulationTime,
    ) {
        // Not implemented for EconSystem
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
