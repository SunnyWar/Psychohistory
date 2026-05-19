// --- Imports ---
use core::app::App;
use core::plugin::Plugin;
use models::{EconState, GovState};
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;

// --- Plugin Struct ---
pub struct GovPlugin;

// --- Plugin Registration ---
impl Plugin for GovPlugin {
    const NAME: &'static str = "gov";

    fn build(&self, app: &mut App) {
        // Insert GovState if not present
        if !app.state.mut_workspace().contains_key("gov") {
            app.state.insert("gov", GovState::default());
        }
        println!("[gov] Plugin build called");
    }
}

// --- Simulation Logic ---
impl SimulationPlugin for GovPlugin {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any + Send + Sync>) {
        let gov = my_state
            .downcast_mut::<GovState>()
            .expect("Failed to downcast to GovState");

        if let Some(econ) = world.get::<EconState>("econ") {
            gov.budget += econ.gdp * gov.tax_rate;
        }
    }
}
