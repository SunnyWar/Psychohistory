//! Legal and legislative system trait and implementations for per-GovType simulation.

use crate::entities::{GovernanceSystem, Legislator, YearOutcome};
use crate::simulation::SimulationState;

pub struct DemocracyModel;
#[derive(Clone)]
struct LawProposal {
    quality: f64,     // [0,1] quality of the law
    support: f64,     // [0,1] support in legislature
    controversy: f64, // [0,1] how controversial
}
pub struct AutocracyModel;
pub struct MonarchyModel;
pub struct OtherModel;
impl LegalSystemModel for DemocracyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome {
        // --- Democratic Legislative Session Simulation ---
        // 1. Propose laws (by legislators, with probability by competence/leadership)
        let proposals = propose_laws(system, state);
        // 2. Committee review (filter/amend proposals)
        let reviewed = committee_review(&proposals, system, state);
        // 3. Debate and amendment (simulate polarization, lobbying, media)
        let debated = debate_and_amend(&reviewed, system, state);
        // 4. Voting in House and Senate (majority, party/faction influence)
        let passed = vote_in_chambers(&debated, system, state);
        // 5. Executive veto/override (simulate chance of veto, override mechanics)
        let enacted = executive_veto(&passed, system, state);
        // 6. Judicial review (chance of law being struck down)
        let final_laws = judicial_review(&enacted, system, state);
        // 7. Update metrics based on quality/quantity of final laws
        let outcome = compute_year_outcome(&final_laws, system, state);
        outcome
    }
}
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
pub trait LegalSystemModel {
    /// Simulate a legislative session (proposal, debate, passage, etc.)
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
    ) -> YearOutcome;
}
// --- Democratic Process Step Stubs ---
fn propose_laws(system: &GovernanceSystem, _state: &mut SimulationState) -> Vec<LawProposal> {
    // Each legislator has a chance to propose a law based on competence and leadership
    let mut proposals = Vec::new();
    for leg in &system.members {
        let prop_chance = 0.2 + 0.3 * leg.competence + 0.2 * leg.leadership_quality;
        if rand::random::<f64>() < prop_chance.min(0.9) {
            proposals.push(LawProposal {
                quality: 0.5 + 0.5 * leg.competence,
                support: 0.4 + 0.3 * leg.representativeness,
                controversy: 0.5 - 0.3 * leg.integrity + 0.2 * (rand::random::<f64>() - 0.5),
            });
        }
    }
    if proposals.is_empty() {
        proposals.push(LawProposal {
            quality: 0.5,
            support: 0.5,
            controversy: 0.5,
        });
    }
    proposals
}

fn committee_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    // TODO: Filter/amend proposals based on committee composition
    proposals.to_vec()
}

fn debate_and_amend(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    // TODO: Simulate debate, lobbying, media, and amendments
    proposals
        .iter()
        .map(|p| LawProposal {
            quality: p.quality * 0.98,
            ..*p
        })
        .collect()
}

fn vote_in_chambers(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    // Simulate party/faction influence: each legislator votes based on support, controversy, and affinity
    let mut passed = Vec::new();
    for prop in proposals {
        let mut yes_votes = 0.0;
        let mut total = 0.0;
        for leg in &system.members {
            let base = prop.support + 0.2 * leg.faction_affinity - 0.1 * prop.controversy;
            let vote = if rand::random::<f64>() < base.clamp(0.0, 1.0) {
                1.0
            } else {
                0.0
            };
            yes_votes += vote;
            total += 1.0;
        }
        if yes_votes / total > 0.5 {
            passed.push(prop.clone());
        }
    }
    passed
}

fn executive_veto(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    // TODO: Simulate veto probability and override
    proposals.to_vec()
}

fn judicial_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    // TODO: Simulate chance of law being struck down
    proposals.to_vec()
}

fn compute_year_outcome(
    final_laws: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> YearOutcome {
    // TODO: Compute metrics based on laws passed, quality, controversy, etc.
    let quality =
        final_laws.iter().map(|l| l.quality).sum::<f64>() / (final_laws.len() as f64).max(1.0);
    YearOutcome {
        law_quality: quality,
        legislative_speed: final_laws.len() as f64 / 10.0,
        ..YearOutcome::default()
    }
}
