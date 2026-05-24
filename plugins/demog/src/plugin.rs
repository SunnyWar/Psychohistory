// plugins/demog/src/plugin.rs
use core::app::App;
use core::plugin::Plugin;
use models::{DemogState, GovState};
use sdk::ReadSnapshot;
use sdk::SimulationPlugin;
use std::any::Any;

pub struct DemogPlugin;

// --- Plugin Registration ---
impl Plugin for DemogPlugin {
    const NAME: &'static str = "demog";

    fn build(&self, app: &mut App) {
        // Insert `DemogState` if not present
        if !app.state.mut_workspace().contains_key("demog") {
            app.state.insert("demog", DemogState::default());
        }
        println!("[demog] Plugin build called");
    }
}

impl SimulationPlugin for DemogPlugin {
    fn name(&self) -> &'static str {
        "demog"
    }

    // Add + Send + Sync here:
    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
    ) {
        let demog = my_state
            .downcast_mut::<DemogState>()
            .expect("Failed to downcast to DemogState");

        let dt = time.delta_years();

        // If you need the key, pass it via the system runner or store in state.
        // For now, fallback to single-country logic or refactor as needed.
        let stability_modifier = if let Some(gov) = world.get::<GovState>("gov") {
            gov.tax_rate.min(0.5)
        } else {
            0.0
        };

        demog.birth_rate = 0.015 - stability_modifier * 0.01;
        demog.population = (demog.population as f64 * (1.0 + demog.birth_rate * dt)) as u64;
    }
}
