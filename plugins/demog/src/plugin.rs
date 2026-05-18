use std::any::Any;
   use models::{DemogState, GovState};
   use sdk::{SimulationPlugin, ReadSnapshot};

   pub struct DemogPlugin;

   impl SimulationPlugin for DemogPlugin {
       fn name(&self) -> &'static str {
           "demog"
       }

       fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any>) {
           let demog = my_state
               .downcast_mut::<DemogState>()
               .expect("Failed to downcast to DemogState");

           // Adjust population growth based on government stability
           let stability_modifier = if let Some(gov) = world.get::<GovState>("gov") {
               gov.tax_rate.min(0.5) // High taxes slow down growth
           } else {
               0.0
           };

           demog.birth_rate = 0.015 - stability_modifier * 0.01;
           demog.population = (demog.population as f64 * (1.0 + demog.birth_rate)) as u64;
       }
   }