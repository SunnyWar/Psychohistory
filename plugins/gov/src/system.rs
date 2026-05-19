// plugins/gov/src/system.rs
use core::{state::SimulationState, system::System, time::SimulationTime};
use models::GovState;
use sdk::{ReadSnapshot, SimulationPlugin};
use crate::plugin::GovPlugin;

pub struct GovSystem;

impl System for GovSystem {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn run(&mut self, state: &mut SimulationState, _time: SimulationTime) {
        let gov = state
            .mut_workspace()
            .get_mut("gov")
            .and_then(|b| b.downcast_mut::<GovState>())
            .expect("Failed to get mutable GovState");

        // placeholder update
        gov.stability = (gov.stability + 0.001).min(1.0);
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn std::any::Any + Send + Sync>,
        _time: SimulationTime,
    ) {
        // Dispatch directly into the parallel simulation execution block
        GovPlugin.step(snapshot, bucket);
    }
}
