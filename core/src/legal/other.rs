use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use sdk::Blackboard;

pub struct OtherModel;

impl LegalSystemModel for OtherModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _blackboard: &Blackboard,
        _year: usize,
        _context: &mut SimulationContext,
    ) -> YearOutcome {
        // TODO: Implement custom/other process
        YearOutcome::default()
    }
}
