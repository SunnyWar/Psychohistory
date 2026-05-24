// plugins/econ/src/plugin.rs
use core::app::App;
use core::plugin::Plugin;
use models::{DemogState, EconState, GovState};
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

        // If you need the key, pass it via the system runner or store in state.
        // For now, fallback to single-country logic or refactor as needed.
        let population = world.get::<DemogState>("demog").map_or(0, |d| d.population);
        let stability = world.get::<GovState>("gov").map_or(1.0, |g| g.stability);

        if population > 10_000_000 {
            // GDP growth scales with population size and societal stability
            let base_per_capita_productivity = 50.0;
            let growth_potential = (population as f64) * base_per_capita_productivity * 0.001 * dt;

            // High stability maximizes growth; inflation acts as a drag
            let stability_drag = stability.clamp(0.1, 1.0);
            let inflation_drag = 1.0 - econ.inflation;
            econ.gdp += growth_potential * stability_drag * inflation_drag;
        }

        // Inflation tracks dynamically based on economic heating
        econ.inflation = 0.015 + (econ.gdp * 0.000_000_000_01 * dt);

        // Example: cross-country lookup (e.g., US econ can read China econ)
        // if let Some(china_econ) = world.get::<EconState>("cn:econ") {
        //     let import_drag = china_econ.inflation * 0.05;
        //     econ.gdp -= econ.gdp * import_drag;
        // }
    }
}
