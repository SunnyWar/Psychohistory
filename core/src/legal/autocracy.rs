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
    ) -> YearOutcome {
        // TODO: Implement autocratic process
        YearOutcome::default()
    }
}
