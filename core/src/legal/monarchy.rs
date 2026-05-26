use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use sdk::Blackboard;

pub struct MonarchyModel;

impl LegalSystemModel for MonarchyModel {
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _blackboard: &Blackboard,
        _year: usize,
        _context: &mut SimulationContext,
    ) -> YearOutcome {
        // TODO: Implement monarchy process
        YearOutcome::default()
    }
}
