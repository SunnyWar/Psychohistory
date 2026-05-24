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

        // stability_modifier = gov.tax_rate.min(0.5) or 0.0 if no gov state
        let stability_modifier = world
            .get::<GovState>("gov")
            .map_or(0.0, |g| g.tax_rate.min(0.5));

        // birth_rate = 0.015 - 0.01 * stability_modifier
        demog.birth_rate = 0.01f64.mul_add(-stability_modifier, 0.015);

        // population = population * (1.0 + birth_rate * dt)
        let growth_factor = 1.0f64.mul_add(demog.birth_rate * dt, 1.0);
        demog.population = (demog.population as f64 * growth_factor) as u64;
    }
}
