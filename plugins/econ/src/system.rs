use crate::state::EconState;
use legion::system;
use sdk::components::{InflationComponent, PopulationComponent, StabilityComponent};

#[system(for_each)]
pub fn econ_system(
    econ: &mut EconState,
    population: &PopulationComponent,
    stability: &StabilityComponent,
    inflation: Option<&InflationComponent>,
) {
    let dt = 1.0; // Replace with time.delta_years() if SimulationTime is available as a resource
    let population = population.0;
    let stability = stability.0;
    let inflation_val = inflation.map(|i| i.0).unwrap_or(econ.inflation);

    if population > 10_000_000 {
        let growth_potential = (population as f64) * 50.0 * 0.001 * dt;
        let stability_drag = stability.clamp(0.1, 1.0);
        let inflation_drag = 1.0 - inflation_val;
        econ.gdp += growth_potential * stability_drag * inflation_drag;
    }
    econ.inflation = 0.000_000_000_01f64.mul_add(econ.gdp * dt, 0.015);
}
