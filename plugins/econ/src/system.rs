use crate::EconState;
use core::{state::SimulationState, system::System, time::SimulationTime};

pub struct EconSystem;

impl System for EconSystem {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn run(&mut self, state: &mut SimulationState, _time: SimulationTime) {
        let econ = state.get_mut::<EconState>("econ");

        econ.gdp *= 1.001;
        econ.inflation *= 0.999;
    }
}
