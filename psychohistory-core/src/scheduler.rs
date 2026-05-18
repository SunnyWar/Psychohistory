use crate::{
    state::SimulationState,
    system::System,
    time::{SimulationTime, TimeGranularity},
};

pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        println!("[core] Registering system: {}", system.name());
        self.systems.push(system);
    }

    pub fn run(&mut self, state: &mut SimulationState, steps: u64) {
        for step in 0..steps {
            println!("[core] === Tick {} ===", step);

            let time = SimulationTime {
                step,
                granularity: TimeGranularity::Step,
            };

            for system in &mut self.systems {
                println!("  -> Running system: {}", system.name());
                system.run(state, time);
            }
        }
    }
}
