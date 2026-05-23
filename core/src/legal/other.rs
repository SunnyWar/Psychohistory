use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState; // <-- Ensure it points to simulation

pub struct OtherModel;

impl LegalSystemModel for OtherModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _state: &mut SimulationState,
        _year: usize,
    ) -> YearOutcome {
        // TODO: Implement custom/other process
        YearOutcome::default()
    }
}
