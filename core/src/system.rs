// core/src/system.rs
use crate::state::SimulationState;
use sdk::ReadSnapshot;
use sdk::SimulationTime;
use std::any::Any;

pub trait System {
    fn name(&self) -> &'static str;
    /// Parallel bucket execution for Scheduler
    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
    );
}

type SystemRunner = Box<dyn Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>) + Send + Sync>;

pub struct ParallelSystem {
    name: &'static str,
    runner: SystemRunner,
}

impl ParallelSystem {
    pub fn new<F>(name: &'static str, runner: F) -> Self
    where
        F: Fn(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>) + Send + Sync + 'static,
    {
        Self {
            name,
            runner: Box::new(runner),
        }
    }
}

impl SimulationState {
    /// Safely run one system using safe disjoint field borrowing
    pub fn run_system<F>(&mut self, key: &'static str, runner: F)
    where
        F: FnOnce(&ReadSnapshot, &mut Box<dyn Any + Send + Sync>),
    {
        // Accessing fields directly allows the borrow checker to split the borrow.
        // `self.current` is borrowed immutably, while `self.next` is borrowed mutably.
        let snapshot = ReadSnapshot::new(&self.current);

        if let Some(my_next_space) = self.next.get_mut(key) {
            runner(&snapshot, my_next_space);
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
    ) {
        (self.runner)(snapshot, bucket);
    }
}
