//! Legal and legislative system trait and implementations for per-GovType simulation.

use crate::entities::{GovernanceSystem, YearOutcome};
use crate::simulation::SimulationState;

pub trait LegalSystemModel {
    /// Simulate a legislative session (proposal, debate, passage, etc.)
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome;
}

pub struct DemocracyModel;
impl LegalSystemModel for DemocracyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome {
        // TODO: Implement detailed democratic process
        YearOutcome::default()
    }
}

pub struct AutocracyModel;
impl LegalSystemModel for AutocracyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome {
        // TODO: Implement autocratic process
        YearOutcome::default()
    }
}

pub struct MonarchyModel;
impl LegalSystemModel for MonarchyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome {
        // TODO: Implement monarchy process
        YearOutcome::default()
    }
}

pub struct OtherModel;
impl LegalSystemModel for OtherModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome {
        // TODO: Implement custom/other process
        YearOutcome::default()
    }
}
