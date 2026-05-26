// plugins/econ/src/plugin.rs
use crate::state::EconState;
use crate::system::econ_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin, SimulationTime};
use std::any::Any;

pub struct EconPlugin;

impl SimulationPlugin for EconPlugin {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
    ) {
        let econ = my_state
            .downcast_mut::<EconState>()
            .expect("Failed to downcast to EconState");
        let dt = time.delta_years();
        let population = world.get::<u64>("demog:population").copied().unwrap_or(0);
        let stability = world.get::<f64>("gov:stability").copied().unwrap_or(1.0);
        if population > 10_000_000 {
            let growth_potential = (population as f64) * 50.0 * 0.001 * dt;
            let stability_drag = stability.clamp(0.1, 1.0);
            let inflation_drag = 1.0 - econ.inflation;
            econ.gdp += growth_potential * stability_drag * inflation_drag;
        }
        econ.inflation = 0.000_000_000_01f64.mul_add(econ.gdp * dt, 0.015);
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(econ_system_system());
    }
}
