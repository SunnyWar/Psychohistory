// plugins/demog/src/plugin.rs
use crate::state::DemogState;
use core::app::App;
use core::plugin::Plugin;
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

        // stability_modifier = gov.tax_rate.min(0.5) or 0.0 if no gov state
        let stability_modifier = world
            .get::<f64>("gov:tax_rate")
            .copied()
            .map(|v| v.min(0.5))
            .unwrap_or(0.0);

        // birth_rate = 0.015 - 0.01 * stability_modifier
        demog.birth_rate = 0.01f64.mul_add(-stability_modifier, 0.015);

        // population = population * (1.0 + birth_rate * dt)
        let dt = time.delta_years();
        let growth_factor = 1.0f64.mul_add(demog.birth_rate * dt, 1.0);
        demog.population = (demog.population as f64 * growth_factor) as u64;

        // Register demog primitives for cross-domain access
        // No direct workspace mutation here; kernel is responsible for publishing demog state
    }
}
