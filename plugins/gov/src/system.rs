use core::{state::SimulationState, system::System, time::SimulationTime};
use models::GovState;

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
}
