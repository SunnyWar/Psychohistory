// plugins/econ/src/plugin.rs
use crate::state::EconState;
use core::app::App;
use core::plugin::Plugin;
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;

// --- Plugin Struct ---
pub struct EconPlugin;

// --- Plugin Registration ---
impl Plugin for EconPlugin {
    const NAME: &'static str = "econ";

    fn build(&self, app: &mut App) {
        // Insert `EconState` if not present
        if !app.state.mut_workspace().contains_key("econ") {
            app.state.insert("econ", EconState::default());
        }
        println!("[econ] Plugin build called");
    }
}
// --- End Plugin Registration ---

// --- Simulation Logic ---
impl SimulationPlugin for EconPlugin {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
    ) {
        let econ = my_state
            .downcast_mut::<EconState>()
            .expect("Failed to downcast to EconState");

        let dt = time.delta_years();

        let population = world.get::<u64>("demog:population").copied().unwrap_or(0);
        let stability = world.get::<f64>("gov:stability").copied().unwrap_or(1.0);

        if population > 10_000_000 {
            // growth_potential = population * base_per_capita_productivity * 0.001 * dt
            let growth_potential = (population as f64) * 50.0 * 0.001 * dt;
            let stability_drag = stability.clamp(0.1, 1.0);
            let inflation_drag = 1.0 - econ.inflation;
            econ.gdp += growth_potential * stability_drag * inflation_drag;
        }

        // inflation = 0.015 + econ.gdp * 0.000_000_000_01 * dt
        econ.inflation = 0.000_000_000_01f64.mul_add(econ.gdp * dt, 0.015);

        // No direct workspace mutation here; kernel is responsible for publishing econ state
    }
}
