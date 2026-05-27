//! Domain-agnostic simulation runner for Psychohistory core
//! Strictly follows open-blackboard architecture and parallel plugin execution

use sdk::{Blackboard, SimulationPlugin, ReadSnapshot, SimulationTime};
use rayon::prelude::*;

/// Advances the simulation by one tick (domain-blind)
///
/// # Arguments
/// - `plugins`: slice of boxed SimulationPlugin trait objects
/// - `snapshot`: immutable execution window (ReadSnapshot)
/// - `blackboard`: shared Blackboard container
/// - `sim_time`: mutable SimulationTime (for tick progression)
///
/// # Returns
/// None. All state is managed via the blackboard and plugins.
pub fn simulate_tick(
    plugins: &[Box<dyn SimulationPlugin + Sync>],
    snapshot: &ReadSnapshot,
    blackboard: &Blackboard,
    sim_time: &mut SimulationTime,
) {
    // Advance simulation time
    sim_time.step += 1;
    // Run all plugins in parallel, each updating the blackboard
    plugins.par_iter().for_each(|plugin| {
        plugin.execute(snapshot, blackboard);
    });
}
