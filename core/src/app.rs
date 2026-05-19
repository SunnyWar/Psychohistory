// core/src/app.rs
use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::SimulationState};
use sdk::TimeGranularity;

pub struct App {
    pub state: SimulationState,
    pub scheduler: Scheduler,
}

impl App {
    /// Safely modifies an initialized component state across both data planes before a run execution.
    pub fn update_state<T: 'static>(&mut self, key: &'static str, mutator: impl FnMut(&mut T)) {
        let mut mutator = mutator;
        self.state.update_initial_state::<T>(key, &mut mutator);
    }
    pub fn new() -> Self {
        Self {
            state: SimulationState::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) {
        println!("[core] Loading plugin: {}", P::NAME);
        plugin.build(self);
    }

    pub fn run(&mut self, steps: u64, granularity: TimeGranularity) {
        self.scheduler.run(&mut self.state, steps, granularity);
    }

    pub fn summarize_state(&self) {
        println!("[core] Final state keys:");
        for key in self.state.keys() {
            println!("  - {}", key);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
