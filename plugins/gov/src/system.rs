// plugins/gov/src/system.rs
use crate::state::GovState;
use legion::system;
use sdk::components::{GdpComponent, InflationComponent, StabilityComponent, TaxRateComponent};

#[system(for_each)]
pub fn gov_system(
    gov: &mut GovState,
    gdp: &GdpComponent,
    inflation: &InflationComponent,
    tax_rate: &mut TaxRateComponent,
    stability: &mut StabilityComponent,
) {
    let dt = 1.0; // Replace with time.delta_years() if SimulationTime is available as a resource
    let gdp = gdp.0;
    let inflation = inflation.0;

    let tax_revenue = gdp * gov.tax_rate * dt;
    gov.budget += tax_revenue;

    let tax_friction = if gov.tax_rate > 0.25 {
        (gov.tax_rate - 0.25) * 0.5 * dt
    } else {
        0.0
    };
    let inflation_friction = inflation * 0.2 * dt;
    let public_spending_stimulus = (gov.budget / gdp).min(0.05) * dt;

    gov.stability = (gov.stability + public_spending_stimulus - tax_friction - inflation_friction)
        .clamp(0.0, 1.0);

    gov.budget -= gov.budget * 0.08 * dt;

    tax_rate.0 = gov.tax_rate;
    stability.0 = gov.stability;
}
