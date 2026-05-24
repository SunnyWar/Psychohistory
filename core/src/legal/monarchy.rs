use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;

pub struct MonarchyModel;

impl LegalSystemModel for MonarchyModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _state: &mut SimulationState,
        _year: usize,
        _context: &mut SimulationContext,
    ) -> YearOutcome {
        // TODO: Implement monarchy process
        YearOutcome::default()
    }
}
