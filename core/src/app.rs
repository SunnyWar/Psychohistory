// core/src/app.rs
use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::SimulationState};
use log::{debug, info};
use sdk::TimeGranularity;
// use std::collections::HashMap;

// Color helpers (used in println macros below)
// const YELLOW: &str = "\x1b[33m";
// const RESET: &str = "\x1b[0m";

pub struct App {
    pub state: SimulationState,
    pub scheduler: Scheduler,
}

impl App {
    /// Safely modifies an initialized component state across both data planes before a run execution.
    pub fn update_state<T: 'static>(&mut self, key: &'static str, mutator: impl FnMut(&mut T)) {
        debug!("Updating state for key: {key}");
        let mut mutator = mutator;
        self.state.update_initial_state::<T>(key, &mut mutator);
    }

    #[must_use]
    pub fn new() -> Self {
        debug!("Creating new App instance");
        let state = SimulationState::new();
        let scheduler = Scheduler::new();
        Self { state, scheduler }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) {
        info!("Loading plugin: {}", P::NAME);
        plugin.build(self);
        debug!("Plugin {} built and registered", P::NAME);
    }

    pub fn run(&mut self, steps: u64, granularity: TimeGranularity) {
        info!("Simulation run starting: steps={steps}, granularity={granularity:?}");
        self.scheduler.run(&mut self.state, steps, granularity);
        info!("Simulation run completed");
    }

    pub fn summarize_state(&self) {
        println!("[core] Final state keys:");
        let keys: Vec<_> = self
            .state
            .keys()
            .map(std::string::ToString::to_string)
            .collect();
        for key in &keys {
            println!("  - {key}");
        }
        // State diffing and system transition reporting removed as part of plugin state decoupling
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
