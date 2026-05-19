use core::app::App;
use core::plugin::Plugin;
impl Plugin for EconPlugin {
    const NAME: &'static str = "econ";
    fn build(&self, app: &mut App) {
        // Insert EconState if not present
        use models::EconState;
        if !app.state.mut_workspace().contains_key("econ") {
            app.state.insert("econ", EconState::default());
        }
        println!("[econ] Plugin build called");
    }
}
use models::{DemogState, EconState};
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;

pub struct EconPlugin;

impl SimulationPlugin for EconPlugin {
    fn name(&self) -> &'static str {
        "econ"
    }

    // Add + Send + Sync here:
    fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any + Send + Sync>) {
        let econ = my_state
            .downcast_mut::<EconState>()
            .expect("Failed to downcast to EconState");

        if let Some(demog) = world.get::<DemogState>("demog") {
            if demog.population > 10_000_000 {
                econ.gdp += 500_000.0;
            }
        }

        econ.inflation = 0.025;
    }
}
