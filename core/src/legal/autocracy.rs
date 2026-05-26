/// Represents a member of the elite cohort in an autocracy.
#[derive(Clone, Debug)]
pub struct Elite {
    pub id: String,
    pub loyalty: f64,    // [0,1] loyalty to the autocrat
    pub influence: f64,  // [0,1] influence over policy
    pub is_active: bool, // false if purged
}

use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use sdk::Blackboard;

/// Represents a direct economic decree issued by the autocrat.
#[derive(Clone, Debug)]
pub struct EconomicDecree {
    /// Tax rate [0, 1]: 0 = no tax, 1 = 100% taxation
    pub tax_rate: f64,
    /// Capital controls [0, 1]: 0 = free flow, 1 = total control
    pub capital_controls: f64,
    /// Trade policy [0, 1]: 0 = free trade, 1 = autarky
    pub trade_policy: f64,
    /// Industrial policy [0, 1]: 0 = laissez-faire, 1 = total state planning
    pub industrial_policy: f64,
    /// Currency regime [0, 1]: 0 = floating, 1 = fixed/pegged
    pub currency_regime: f64,
    /// Resource allocation [0, 1]: 0 = market, 1 = full state allocation
    pub resource_allocation: f64,
    /// Year decree was issued
    pub year: usize,
}

pub struct AutocracyModel;

impl LegalSystemModel for AutocracyModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _blackboard: &Blackboard,
        _year: usize,
        _context: &mut SimulationContext,
    ) -> YearOutcome {
        // TODO: Migrate to blackboard model. For now, this is a stub to satisfy the trait.
        crate::entities::YearOutcome::default()
    }
}
