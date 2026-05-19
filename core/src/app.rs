// core/src/app.rs
use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::SimulationState};
use sdk::TimeGranularity;

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum DomainState {
    Econ(models::EconState),
    Gov(models::GovState),
    Demog(models::DemogState),
    Unknown(String),
}

impl Debug for DomainState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainState::Econ(e) => write!(f, "{:?}", e),
            DomainState::Gov(g) => write!(f, "{:?}", g),
            DomainState::Demog(d) => write!(f, "{:?}", d),
            DomainState::Unknown(s) => write!(f, "{:?}", s),
        }
    }
}

pub struct App {
    pub state: SimulationState,
    pub scheduler: Scheduler,
    pub initial_state: HashMap<String, DomainState>,
}

impl App {
    /// Safely modifies an initialized component state across both data planes before a run execution.
    pub fn update_state<T: 'static>(&mut self, key: &'static str, mutator: impl FnMut(&mut T)) {
        let mut mutator = mutator;
        self.state.update_initial_state::<T>(key, &mut mutator);
    }
    pub fn new() -> Self {
        let state = SimulationState::new();
        let scheduler = Scheduler::new();
        let initial_state = HashMap::new();
        Self {
            state,
            scheduler,
            initial_state,
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) {
        println!("[core] Loading plugin: {}", P::NAME);
        plugin.build(self);
    }

    /// Capture a snapshot of the current state as a HashMap<String, DomainState>
    pub fn snapshot_state(&self) -> HashMap<String, DomainState> {
        let mut map = HashMap::new();
        for (&key, val) in self.state.as_raw_map() {
            let key_str = key.to_string();
            if let Some(econ) = val.downcast_ref::<models::EconState>() {
                map.insert(key_str, DomainState::Econ(econ.clone()));
            } else if let Some(gov) = val.downcast_ref::<models::GovState>() {
                map.insert(key_str, DomainState::Gov(gov.clone()));
            } else if let Some(demog) = val.downcast_ref::<models::DemogState>() {
                map.insert(key_str, DomainState::Demog(demog.clone()));
            } else {
                map.insert(key_str, DomainState::Unknown("<untracked>".to_string()));
            }
        }
        map
    }

    pub fn run(&mut self, steps: u64, granularity: TimeGranularity) {
        // Capture initial snapshot before any ticks
        self.initial_state = self.snapshot_state();
        self.scheduler.run(&mut self.state, steps, granularity);
    }

    pub fn summarize_state(&self) {
        // (removed unused import)
        println!("[core] Final state keys:");
        let mut keys: Vec<_> = self.state.keys().map(|k| k.to_string()).collect();
        keys.sort();
        for key in &keys {
            println!("  - {}", key);
        }
        println!("[core] State changes:");
        let final_snapshot = self.snapshot_state();
        for key in &keys {
            let before = self.initial_state.get(key);
            let after = final_snapshot.get(key);
            if let (Some(before), Some(after)) = (before, after) {
                if let Some(diff) = diff_states(before, after) {
                    println!("{}: {}", key, diff);
                }
            }
        }
    }
}

/// Returns None if equal, or Some("before → after") if different
fn diff_states<T: Debug + PartialEq>(before: &T, after: &T) -> Option<String> {
    if before == after {
        None
    } else {
        Some(format!("{:?} → {:?}", before, after))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
