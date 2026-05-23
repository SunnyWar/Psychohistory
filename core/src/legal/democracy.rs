use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;

pub struct DemocracyModel;

#[derive(Clone)]
struct LawProposal {
    quality: f64,     // [0,1] quality of the law
    support: f64,     // [0,1] support in legislature
    controversy: f64, // [0,1] how controversial
}

impl LegalSystemModel for DemocracyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        _year: usize,
    ) -> YearOutcome {
        // --- Democratic Legislative Session Simulation ---
        let proposals = propose_laws(system, state);
        let reviewed = committee_review(&proposals, system, state);
        let debated = debate_and_amend(&reviewed, system, state);
        let passed = vote_in_chambers(&debated, system, state);
        let enacted = executive_veto(&passed, system, state);
        let final_laws = judicial_review(&enacted, system, state);

        compute_year_outcome(&final_laws, system, state)
    }
}

// --- Democratic Process Step Stubs ---

fn propose_laws(system: &GovernanceSystem, _state: &mut SimulationState) -> Vec<LawProposal> {
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
    proposals.to_vec()
}

fn debate_and_amend(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
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
    proposals.to_vec()
}

fn judicial_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> Vec<LawProposal> {
    proposals.to_vec()
}

fn compute_year_outcome(
    final_laws: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
) -> YearOutcome {
    let quality =
        final_laws.iter().map(|l| l.quality).sum::<f64>() / (final_laws.len() as f64).max(1.0);
    YearOutcome {
        law_quality: quality,
        legislative_speed: final_laws.len() as f64 / 10.0,
        ..YearOutcome::default()
    }
}
