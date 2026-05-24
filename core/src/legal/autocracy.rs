use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;

pub struct AutocracyModel;

impl LegalSystemModel for AutocracyModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _state: &mut SimulationState,
        _year: usize,
        _context: &mut SimulationContext,
    ) -> YearOutcome {
        // TODO: Implement autocratic process
        YearOutcome::default()
    }
}
