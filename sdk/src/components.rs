/// Shared ECS components for cross-domain metrics.
/// These are used for flat, decoupled data access between plugins.

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TaxRateComponent(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct StabilityComponent(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PopulationComponent(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct InflationComponent(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GdpComponent(pub f64);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BirthRateComponent(pub f64);
