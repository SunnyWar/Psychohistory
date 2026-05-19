use core::{system::System, time::SimulationTime};
use sdk::ReadSnapshot;
use std::any::Any;

pub struct DemogSystem;

impl System for DemogSystem {
	fn name(&self) -> &'static str {
		"demog"
	}

	fn run(&mut self, _state: &mut core::state::SimulationState, _time: SimulationTime) {
		// Not implemented
	}

	fn run_system(&self, _snapshot: &ReadSnapshot, _bucket: &mut Box<dyn Any + Send + Sync>, _time: SimulationTime) {
		// Not implemented for DemogSystem
	}
}
// plugins/demog/src/system.rs
