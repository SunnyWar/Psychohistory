// core/src/system.rs
use crate::state::SimulationState;
use log::{debug, error};
use sdk::ReadSnapshot;
use sdk::SimulationTime;
use std::any::Any;

type SystemRunner = Box<dyn Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>) + Send + Sync>;

pub struct ParallelSystem {
    name: &'static str,
    runner: SystemRunner,
}

impl ParallelSystem {
    pub fn new<F>(name: &'static str, runner: F) -> Self
    where
        F: Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>, &'static str) + Send + Sync + 'static,
    {
        Self {
            name,
            runner: Box::new(move |snapshot, bucket| runner(snapshot, bucket, name)),
        }
    }
}

impl SimulationState {
    /// Safely run one system using safe disjoint field borrowing
    pub fn run_system<F>(&mut self, key: &'static str, runner: F)
    where
        F: FnOnce(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>),
    {
        let snapshot = ReadSnapshot::new(&self.current);

        match self.next.get_mut(key) {
            Some(my_next_space) => {
                debug!("Running system for key: {key}");
                runner(&snapshot, my_next_space);
            }
            None => {
                error!("No mutable state bucket found for key: {key}");
            }
        }
    }
}

impl System for ParallelSystem {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        _time: SimulationTime,
        _key: &'static str,
    ) {
        (self.runner)(snapshot, bucket);
    }
}

pub trait System {
    fn name(&self) -> &'static str;
    /// Parallel bucket execution for Scheduler
    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
        _key: &'static str,
    );
}
