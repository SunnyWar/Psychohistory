// plugins/gov/src/plugin.rs
use crate::state::GovState;
use crate::system::gov_system_system;
use legion::systems::Builder as ScheduleBuilder;
use sdk::{ReadSnapshot, SimulationPlugin, SimulationTime};
use std::any::Any;

pub struct GovPlugin;

impl SimulationPlugin for GovPlugin {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn step(
        &self,
        world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
    ) {
        let gov = my_state
            .downcast_mut::<GovState>()
            .expect("Failed to downcast to GovState");
        let dt = time.delta_years();
        let gdp = world.get::<f64>("econ:gdp").copied().unwrap_or(0.0);
        let inflation = world.get::<f64>("econ:inflation").copied().unwrap_or(0.0);
        let tax_revenue = gdp * gov.tax_rate * dt;
        gov.budget += tax_revenue;
        let tax_friction = if gov.tax_rate > 0.25 {
            (gov.tax_rate - 0.25) * 0.5 * dt
        } else {
            0.0
        };
        let inflation_friction = inflation * 0.2 * dt;
        let public_spending_stimulus = (gov.budget / gdp).min(0.05) * dt;
        gov.stability =
            (gov.stability + public_spending_stimulus - tax_friction - inflation_friction)
                .clamp(0.0, 1.0);
        gov.budget -= gov.budget * 0.08 * dt;
    }

    fn register_systems(&self, schedule: &mut ScheduleBuilder) {
        schedule.add_system(gov_system_system());
    }
}
