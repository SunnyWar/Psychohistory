// plugins/demog/src/plugin.rs
use crate::state::DemogState;
use crate::system::demog_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin, SimulationTime};
use std::any::Any;

pub struct DemogPlugin;

impl SimulationPlugin for DemogPlugin {
    fn name(&self) -> &'static str {
        "demog"
    }

    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
    ) {
        let demog = my_state
            .downcast_mut::<DemogState>()
            .expect("Failed to downcast to DemogState");
        let stability_modifier = world
            .get::<f64>("gov:tax_rate")
            .copied()
            .map(|v| v.min(0.5))
            .unwrap_or(0.0);
        demog.birth_rate = 0.01f64.mul_add(-stability_modifier, 0.015);
        let dt = time.delta_years();
        let growth_factor = 1.0f64.mul_add(demog.birth_rate * dt, 1.0);
        demog.population = (demog.population as f64 * growth_factor) as u64;
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(demog_system_system());
    }
}
