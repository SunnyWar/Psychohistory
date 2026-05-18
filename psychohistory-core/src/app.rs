use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::DomainRegistry};

pub struct App {
    pub state: DomainRegistry,
    pub scheduler: Scheduler,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: DomainRegistry::new(),
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
