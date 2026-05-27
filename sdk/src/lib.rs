pub mod components;
pub mod influence;
use std::any::Any;
// (already imported above if needed)

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

use std::collections::HashMap;
use std::sync::RwLock;

/// Blackboard: Open, thread-safe, dynamic key-value metric store for inter-model communication.
/// This enables arbitrary metric passing between models and systems.
///
/// _[Theory: See open blackboard architectures in distributed AI and system dynamics literature]_
#[derive(Debug, Default)]
pub struct Blackboard {
    /// Thread-safe dynamic string-to-f64 storage
    metrics: RwLock<HashMap<String, f64>>,
}

impl Blackboard {
    /// Create a new, empty blackboard
    pub fn new() -> Self {
        Self {
            metrics: RwLock::new(HashMap::new()),
        }
    }

    /// Sets or updates a dynamic metric value
    pub fn set(&self, key: &str, value: f64) {
        if let Ok(mut map) = self.metrics.write() {
            map.insert(key.to_string(), value);
        }
    }

    /// Retrieves a metric value, safely defaulting to 0.0 if not initialized
    pub fn get(&self, key: &str) -> f64 {
        self.metrics
            .read()
            .ok()
            .and_then(|map| map.get(key).copied())
            .unwrap_or(0.0)
    }
}

// Make sure Blackboard is safely cloneable using Arc internally or implement explicit clone
impl Clone for Blackboard {
    fn clone(&self) -> Self {
        if let Ok(map) = self.metrics.read() {
            Self {
                metrics: RwLock::new(map.clone()),
            }
        } else {
            Self::new()
        }
    }
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
    /// Returns the plugin's name
    fn name(&self) -> &'static str;

    /// Domain-blind execution method for open-blackboard simulation
    fn execute(&self, snapshot: &ReadSnapshot, blackboard: &Blackboard);

    /// Register all ECS systems for this plugin into the kernel schedule.
    fn register_systems(&self, schedule: &mut ScheduleBuilder);
}
