use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;
use rand::Rng;
use rand::RngExt;

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
        context: &mut SimulationContext,
    ) -> YearOutcome {
        // --- Democratic Legislative Session Simulation ---
        let proposals = propose_laws(system, state, context);
        let reviewed = committee_review(&proposals, system, state, context);
        let debated = debate_and_amend(&reviewed, system, state, context);
        let passed = vote_in_chambers(&debated, system, state, context);
        let enacted = executive_veto(&passed, system, state, context);
        let final_laws = judicial_review(&enacted, system, state, context);

        compute_year_outcome(&final_laws, system, state)
    }
}

// --- Democratic Process Step Stubs ---

// BUG: these use the global random generator, which can cause non-determinism in multi-threaded contexts. Should use a thread-local RNG seeded from the main seed for full determinism.
fn propose_laws(
    system: &GovernanceSystem,
    _state: &SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let mut proposals = Vec::with_capacity(system.members.len());

    for leg in &system.members {
        // prop_chance = 0.2 + 0.3 * competence + 0.2 * leadership_quality
        let prop_chance = 0.3f64
            .mul_add(leg.competence, 0.2f64.mul_add(leg.leadership_quality, 0.2))
            .min(0.9);

        let random_f64: f64 = context.rand.random();

        if random_f64 < prop_chance {
            // quality = 0.5 + 0.5 * competence
            let quality = 0.5f64.mul_add(leg.competence, 0.5);

            // support = 0.4 + 0.3 * representativeness
            let support = 0.3f64.mul_add(leg.representativeness, 0.4);

            // controversy = 0.5 - 0.3 * integrity + 0.2 * (rand - 0.5)
            let raw_u64 = context.rand.next_u64();
            let noise = (raw_u64 as f64) / (u64::MAX as f64) - 0.5;
            let controversy = 0.3f64.mul_add(-leg.integrity, 0.2f64.mul_add(noise, 0.5));

            proposals.push(LawProposal {
                quality,
                support,
                controversy,
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
    _context: &mut SimulationContext,
) -> Vec<LawProposal> {
    proposals.to_vec()
}

/// Simulate legislative debate and amendment process.
///
/// Debate can shift support and controversy, and amendments can nonlinearly affect law quality.
///
/// Debate/Amendment Model:
///   [
///         ext{new\_support} = \sigma(s + \Delta_{debate})
///         ext{new\_quality} = q \cdot (1 + \alpha_{amend} - \beta_{controversy})
///         ext{new\_controversy} = c + \gamma_{debate}
///   ]
/// Where $\sigma$ is a sigmoid, $\Delta_{debate}$ is a random/faction-driven shift, $\alpha_{amend}$ is amendment effect, $\beta_{controversy}$ is penalty for controversy, $\gamma_{debate}$ is debate noise.
/// Theory: Deliberative democracy (Habermas, 1996); see also "Legislative Bargaining and Amendment Dynamics" (Baron & Ferejohn, 1989).
fn debate_and_amend(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    _state: &mut SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    proposals
        .iter()
        .map(|p| {
            // Debate shift: random + average faction affinity
            let avg_faction: f64 = system.members.iter().map(|l| l.faction_affinity).sum::<f64>() / (system.members.len() as f64).max(1.0);
            let debate_noise = (context.rand.random::<f64>() - 0.5) * 0.1;
            let delta_debate = 0.05 * avg_faction + debate_noise;

            // Amendment effect: random, can be positive or negative, scaled by controversy
            let amend_effect = (context.rand.random::<f64>() - 0.5) * 0.08 * (1.0 - p.controversy);

            // Controversy penalty: higher controversy reduces quality
            let controversy_penalty = 0.04 * p.controversy;

            // New support: sigmoid of original plus debate shift
            let new_support = 1.0 / (1.0 + (-((p.support + delta_debate) * 3.0 - 1.5)).exp());

            // New quality: non-linear effect of amendment and controversy
            let new_quality = (p.quality * (1.0 + amend_effect - controversy_penalty)).clamp(0.0, 1.0);

            // New controversy: add debate noise
            let new_controversy = (p.controversy + debate_noise).clamp(0.0, 1.0);

            LawProposal {
                quality: new_quality,
                support: new_support,
                controversy: new_controversy,
            }
        })
        .collect()
}

// BUG: the uses the global random generator, which can cause non-determinism in multi-threaded contexts. Should use a thread-local RNG seeded from the main seed for full determinism.
fn vote_in_chambers(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    _state: &SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let member_count = system.members.len() as f64;

    let mut passed = Vec::with_capacity(proposals.len());

    for prop in proposals {
        let mut yes_votes = 0.0;

        // base_support = prop.support - 0.1 * prop.controversy
        let base_support = 0.1f64.mul_add(-prop.controversy, prop.support);

        for leg in &system.members {
            // p = base_support + 0.2 * leg.faction_affinity
            let p = 0.2f64
                .mul_add(leg.faction_affinity, base_support)
                .clamp(0.0, 1.0);

            let random_f64: f64 = context.rand.random();

            if random_f64 < p {
                yes_votes += 1.0;
            }
        }

        if yes_votes > member_count * 0.5 {
            passed.push(prop.clone());
        }
    }

    passed
}

fn executive_veto(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
    _context: &mut SimulationContext,
) -> Vec<LawProposal> {
    proposals.to_vec()
}

fn judicial_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
    _context: &mut SimulationContext,
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
