use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::SimulationState};

pub struct App {
    pub state: SimulationState,
    pub scheduler: Scheduler,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: SimulationState::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) {
        println!("[core] Loading plugin: {}", P::NAME);
        plugin.build(self);
    }

    pub fn run(&mut self, steps: u64) {
        self.scheduler.run(&mut self.state, steps);
    }

    pub fn summarize_state(&self) {
        println!("[core] Final state keys:");
        for key in self.state.keys() {
            println!("  - {}", key);
        }
    }
}
