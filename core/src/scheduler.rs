use crate::{
    state::SimulationState,
    system::System,
    time::{SimulationTime, TimeGranularity},
};

use indicatif::{ProgressBar, ProgressStyle};

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
        let pb = ProgressBar::new(steps);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ticks ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        for step in 0..steps {
            let time = SimulationTime {
                step,
                granularity: TimeGranularity::Step,
            };

            for system in &mut self.systems {
                system.run(state, time);
            }

            pb.inc(1);
        }

        pb.finish_with_message("Simulation complete");
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
