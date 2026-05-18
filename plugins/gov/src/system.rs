use crate::GovState;
use core::{state::SimulationState, system::System, time::SimulationTime};

pub struct GovSystem;

impl System for GovSystem {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn run(&mut self, state: &mut SimulationState, _time: SimulationTime) {
        let gov = state.get_mut::<GovState>("gov");

        // placeholder update
        gov.stability = (gov.stability + 0.001).min(1.0);
    }
}
