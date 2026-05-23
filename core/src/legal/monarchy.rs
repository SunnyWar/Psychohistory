use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState; // <-- Add this line

pub struct MonarchyModel;

impl LegalSystemModel for MonarchyModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _state: &mut SimulationState,
        _year: usize,
    ) -> YearOutcome {
        // TODO: Implement monarchy process
        YearOutcome::default()
    }
}
