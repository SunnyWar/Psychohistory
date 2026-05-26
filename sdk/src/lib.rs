pub mod components;
pub mod influence;
use std::any::Any;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeGranularity {
    Step,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Clone, Copy, Debug)]
pub struct SimulationTime {
    pub step: u64,
    pub granularity: TimeGranularity,
}

pub struct ReadSnapshot<'a> {
    inner: &'a HashMap<&'static str, Box<dyn Any + Send + Sync>>,
}

impl SimulationTime {
    /// Returns the fractional portion of a calendar year represented by a single simulation step.
    pub fn delta_years(&self) -> f64 {
        match self.granularity {
            TimeGranularity::Step | TimeGranularity::Yearly => 1.0,
            TimeGranularity::Quarterly => 0.25,
            TimeGranularity::Monthly => 1.0 / 12.0,
        }
    }
}

impl<'a> ReadSnapshot<'a> {
    pub fn new(inner: &'a HashMap<&'static str, Box<dyn Any + Send + Sync>>) -> Self {
        Self { inner }
    }

    pub fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.inner.get(key)?.downcast_ref::<T>()
    }

    // All plugin-specific field extraction logic removed. Only generic get<T> is supported.
}

use legion::systems::Builder as ScheduleBuilder;

pub trait SimulationPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
    );

    /// Register all ECS systems for this plugin into the kernel schedule.
    fn register_systems(&self, schedule: &mut ScheduleBuilder);
}
