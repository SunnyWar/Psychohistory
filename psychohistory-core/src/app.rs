use crate::{scheduler::Scheduler, state::SimulationState};
use crate::plugin::Plugin;

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
        plugin.build(self);
    }

    pub fn run(&mut self, steps: u64) {
        self.scheduler.run(&mut self.state, steps);
    }
}
