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
        // Insert EconState if not present
        if !app.state.mut_workspace().contains_key("econ") {
            app.state.insert("econ", EconState::default());
        }
        println!("[econ] Plugin build called");
    }
}

// --- Simulation Logic ---
impl SimulationPlugin for EconPlugin {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any + Send + Sync>) {
        let econ = my_state
            .downcast_mut::<EconState>()
            .expect("Failed to downcast to EconState");

        // Fetch cross-cutting variables from the frozen read snapshot
        let population = world
            .get::<DemogState>("demog")
            .map(|d| d.population)
            .unwrap_or(0);
        let stability = world
            .get::<GovState>("gov")
            .map(|g| g.stability)
            .unwrap_or(1.0);

        if population > 10_000_000 {
            // GDP growth scales with population size and societal stability
            let base_per_capita_productivity = 50.0;
            let growth_potential = (population as f64) * base_per_capita_productivity * 0.001;

            // High stability maximizes growth; inflation acts as a drag
            let stability_drag = stability.clamp(0.1, 1.0);
            let inflation_drag = 1.0 - econ.inflation;

            econ.gdp += growth_potential * stability_drag * inflation_drag;
        }

        // Inflation tracks dynamically based on economic heating
        econ.inflation = 0.015 + (econ.gdp * 0.000_000_000_01);
    }
}
