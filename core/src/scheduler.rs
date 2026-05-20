// core/src/scheduler.rs
use crate::{state::SimulationState, system::System};
use sdk::{SimulationTime, TimeGranularity};

use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

pub struct Scheduler {
    // Group systems by the state key they are responsible for modifying
    systems: HashMap<&'static str, Box<dyn System + Send + Sync>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    // Accept key for O(1) dispatch
    pub fn add_system(&mut self, key: &'static str, system: Box<dyn System + Send + Sync>) {
        println!(
            "[core] Registering system for key '{}': {}",
            key,
            system.name()
        );
        self.systems.insert(key, system);
    }

    pub fn run(&mut self, state: &mut SimulationState, steps: u64, granularity: TimeGranularity) {
        let pb = ProgressBar::new(steps);
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ticks ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        for step in 0..steps {
            let time = SimulationTime { step, granularity };

            let systems_ref = &self.systems;
            state.par_execute_systems(|snapshot, key, data_bucket| {
                // Route deeply nested namespaced keys (e.g., "us:ca:los_angeles:econ" -> "econ")
                let archetype_key = key.split(':').next_back().unwrap_or(key);
                if let Some(system) = systems_ref.get(archetype_key) {
                    system.run_system(snapshot, data_bucket, time, key);
                }
            });

            state.advance_tick();
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
