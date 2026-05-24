// plugins/gov/src/plugin.rs
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
        // Insert `GovState` if not present
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

    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
    ) {
        let gov = my_state
            .downcast_mut::<GovState>()
            .expect("Failed to downcast to GovState");

        let dt = time.delta_years();

        if let Some(econ) = world.get::<EconState>("econ") {
            // Tax collection increments the treasury budget
            let tax_revenue = econ.gdp * gov.tax_rate * dt;
            gov.budget += tax_revenue;

            // Stability changes dynamically based on structural factors:
            // 1. High tax rates cause public friction (e.g., taxes > 25% degrade stability)
            let tax_friction = if gov.tax_rate > 0.25 {
                (gov.tax_rate - 0.25) * 0.5 * dt
            } else {
                0.0
            };

            // 2. High inflation hurts quality of life
            let inflation_friction = econ.inflation * 0.2 * dt;

            // 3. Re-investing budget surplus boosts stability
            let public_spending_stimulus = (gov.budget / econ.gdp).min(0.05) * dt;

            gov.stability =
                (gov.stability + public_spending_stimulus - tax_friction - inflation_friction)
                    .clamp(0.0, 1.0);
            // Draw down some budget for societal maintenance costs
            gov.budget -= gov.budget * 0.08 * dt;
        }
    }
}
