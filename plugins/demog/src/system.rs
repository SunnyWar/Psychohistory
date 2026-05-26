use crate::state::DemogState;
use legion::system;
use sdk::components::{BirthRateComponent, PopulationComponent, TaxRateComponent};

#[system(for_each)]
pub fn demog_system(
    demog: &mut DemogState,
    tax_rate: &TaxRateComponent,
    population: &mut PopulationComponent,
    birth_rate: &mut BirthRateComponent,
) {
    let dt = 1.0; // Replace with time.delta_years() if SimulationTime is available as a resource
    let stability_modifier = tax_rate.0.min(0.5);
    demog.birth_rate = 0.01f64.mul_add(-stability_modifier, 0.015);
    let growth_factor = 1.0f64.mul_add(demog.birth_rate * dt, 1.0);
    demog.population = (demog.population as f64 * growth_factor) as u64;
    population.0 = demog.population;
    birth_rate.0 = demog.birth_rate;
}
