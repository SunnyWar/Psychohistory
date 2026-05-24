// core/src/app.rs
use crate::plugin::Plugin;
use crate::{scheduler::Scheduler, state::SimulationState};
use log::{debug, info};
use sdk::TimeGranularity;
use std::collections::HashMap;

// Color helpers (used in println macros below)
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

#[derive(Clone, PartialEq, Debug)]
pub enum DomainState {
    Econ(models::EconState),
    Gov(models::GovState),
    Demog(models::DemogState),
    Unknown(String),
}

pub struct App {
    pub state: SimulationState,
    pub scheduler: Scheduler,
    pub initial_state: HashMap<String, DomainState>,
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
        let initial_state = HashMap::new();
        Self {
            state,
            scheduler,
            initial_state,
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) {
        info!("Loading plugin: {}", P::NAME);
        plugin.build(self);
        debug!("Plugin {} built and registered", P::NAME);
    }

    /// Capture a snapshot of the current state as a `HashMap`<String, `DomainState`>
    #[must_use]
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
        // Add system types as special keys for diffing
        map.insert(
            "__gov_type".to_string(),
            DomainState::Unknown(format!("GOVTYPE::{:?}", self.state.gov_type)),
        );
        map.insert(
            "__econ_system".to_string(),
            DomainState::Unknown(format!("ECONSYSTEM::{:?}", self.state.econ_system)),
        );
        map
    }

    pub fn run(&mut self, steps: u64, granularity: TimeGranularity) {
        info!("Simulation run starting: steps={steps}, granularity={granularity:?}");
        // Capture initial snapshot before any ticks
        self.initial_state = self.snapshot_state();
        debug!("Initial state snapshot captured");
        self.scheduler.run(&mut self.state, steps, granularity);
        info!("Simulation run completed");
    }

    pub fn summarize_state(&self) {
        println!("[core] Final state keys:");
        let keys: Vec<_> = self.state.keys().map(std::string::ToString::to_string).collect();
        for key in &keys {
            println!("  - {key}");
        }
        println!("[core] State changes:");
        let final_snapshot = self.snapshot_state();

        // Track system transitions (global, not per-entity)
        // Assume gov_type and econ_system are global fields in SimulationState
        // Compare before/after by capturing them before and after run
        // For now, just print if changed globally
        // (If you want per-entity, refactor SimulationState)
        // Get initial and final system types
        let before_gov_type = self.initial_state.get("__gov_type");
        let after_gov_type = final_snapshot.get("__gov_type");
        let before_econ_system = self.initial_state.get("__econ_system");
        let after_econ_system = final_snapshot.get("__econ_system");

        if before_gov_type != after_gov_type {
            println!(
                "{YELLOW}[system transition]{RESET} Government type changed: {before_gov_type:?} → {after_gov_type:?}"
            );
        }
        if before_econ_system != after_econ_system {
            println!(
                "{YELLOW}[system transition]{RESET} Economic system changed: {before_econ_system:?} → {after_econ_system:?}"
            );
        }

        for key in &keys {
            let before = self.initial_state.get(key);
            let after = final_snapshot.get(key);
            let mut any_change = false;
            let mut cross_domain = false;
            let mut changes = vec![];
            if let (Some(DomainState::Demog(b)), Some(DomainState::Demog(a))) = (before, after)
                && b != a
            {
                any_change = b.print_diff(a);
                changes.push("demog");
            }
            if let (Some(DomainState::Econ(b)), Some(DomainState::Econ(a))) = (before, after)
                && b != a
            {
                any_change = b.print_diff(a) || any_change;
                changes.push("econ");
            }
            if let (Some(DomainState::Gov(b)), Some(DomainState::Gov(a))) = (before, after)
                && b != a
            {
                any_change = b.print_diff(a) || any_change;
                changes.push("gov");
            }
            // System transitions (global, not per-entity)
            // If you want per-entity, you must store gov_type/econ_system per entity
            // For now, just print if changed globally
            // (Assume initial_state is from before run, so compare to after run)
            // TODO: If you want per-entity, refactor SimulationState
            // Cross-domain effect: more than one domain changed for this key
            if changes.len() > 1 {
                cross_domain = true;
            }
            if any_change {
                println!("\n[{key}]");
                if cross_domain {
                    println!(
                        "  {YELLOW}[cross-domain effect]{RESET} changes in: {:?}",
                        changes
                    );
                }
            }
        }
        // Report system transitions (global)
        // (Assume initial_state is from before run, so compare to after run)
        // If you want per-entity, refactor SimulationState
        // For now, just print if changed globally
        // TODO: If you want per-entity, refactor SimulationState
        // (No-op if unchanged)
        // (This is a placeholder; actual logic may need to compare snapshots)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
