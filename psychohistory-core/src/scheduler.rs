use crate::{state::SimulationState, system::{self, System}, time::{SimulationTime, TimeGranularity}};

pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, sys: Box<dyn System>) {
        self.systems.push(sys);
    }

    pub fn run(&mut self, state: &mut SimulationState, steps: u64) {
        for step in 0..steps {
            let time = SimulationTime {
                step,
                granularity: TimeGranularity::Monthly,
            };
            for sys in self.systems.iter_mut() {
                sys.run(state, time);
            }
        }
    }
}
