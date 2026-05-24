//! Legal and legislative system trait and implementations for per-GovType simulation.

pub mod autocracy;
pub mod democracy;
pub mod monarchy;
pub mod other;

use crate::entities::{GovernanceSystem, YearOutcome};
use crate::simulation::SimulationState;

pub use autocracy::AutocracyModel;
pub use democracy::DemocracyModel;
pub use monarchy::MonarchyModel;
pub use other::OtherModel;

pub trait LegalSystemModel {
    /// Simulate a legislative session (proposal, debate, passage, etc.)
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome;
}
