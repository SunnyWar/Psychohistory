use crate::{state::SimulationState, time::SimulationTime};
use sdk::ReadSnapshot;
use std::any::Any;

pub trait System {
    fn name(&self) -> &'static str;
    fn run(&mut self, state: &mut SimulationState, time: SimulationTime);
}

pub struct ParallelSystem {
    name: &'static str,
    key: &'static str,
    runner: Box<dyn Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>) + Send + Sync>,
}

impl ParallelSystem {
    pub fn new<F>(name: &'static str, key: &'static str, runner: F) -> Self
    where
        F: Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>) + Send + Sync + 'static,
    {
        Self {
            name,
            key,
            runner: Box::new(runner),
        }
    }
}

impl SimulationState {
    /// Safely run one system with correct borrow ordering
    pub fn run_system<F>(&mut self, key: &'static str, runner: F)
    where
        F: FnOnce(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>),
    {
        // 1. Clone references for snapshot to break borrow dependency
        let temp_map: std::collections::HashMap<&'static str, Box<dyn Any + Send + Sync>> =
            self.as_raw_map()
                .iter()
                .map(|(&k, v)| {
                    let cloner = self.cloners().get(&k).expect("Cloner not found");
                    (k, cloner(v))
                })
                .collect();
        let snapshot = ReadSnapshot::new(&temp_map);

        // 2. Now only mutable borrow is active
        if let Some(my_next_space) = self.mut_workspace().get_mut(key) {
            runner(&snapshot, my_next_space);
        }

        // No advance_tick here - caller should do it
    }
}

impl System for ParallelSystem {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&mut self, state: &mut SimulationState, _time: SimulationTime) {
        let temp_map: std::collections::HashMap<&'static str, Box<dyn Any + Send + Sync>> =
            state.as_raw_map()
                .iter()
                .map(|(&k, v)| {
                    let cloner = state.cloners().get(&k).expect("Cloner not found");
                    (k, cloner(v))
                })
                .collect();
        let world_snapshot = ReadSnapshot::new(&temp_map);
        let runner_fn = &self.runner;
        let target_key = self.key;

        if let Some(my_next_space) = state.mut_workspace().get_mut(target_key) {
            (runner_fn)(&world_snapshot, my_next_space);
        }

        state.advance_tick();
    }
}