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
        let (passed, gridlock) = vote_in_chambers(&debated, system, context);
        state.is_gridlocked = gridlock;
        state.is_gridlocked = gridlock;
        let enacted = executive_veto(&passed, system, state, context);
        let final_laws = judicial_review(&enacted, system, state, context);

        compute_year_outcome(&final_laws, system, state)
    }
}

// --- Democratic Process Step Stubs ---

fn propose_laws(
    system: &GovernanceSystem,
    _state: &SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let mut proposals = Vec::with_capacity(system.members.len());

    // Gather global influences
    let lobbying_strength = context.config.lobbying_strength;
    let media_strength = context.config.media_influence_strength;
    let partisan_polarization = context.config.partisan_polarization;
    // For now, public opinion is proxied by public_trust (could be extended)
    let public_opinion = context.config.baseline_public_trust;

    for leg in &system.members {
        // prop_chance = 0.2 + 0.3 * competence + 0.2 * leadership_quality + 0.1 * faction_affinity + 0.08 * lobbying_strength + 0.08 * media_strength + 0.08 * public_opinion
        let prop_chance = 0.3f64.mul_add(
            leg.competence,
            0.2f64.mul_add(
                leg.leadership_quality,
                0.1f64.mul_add(leg.faction_affinity, 0.2),
            ),
        ) + 0.08 * lobbying_strength
            + 0.08 * media_strength
            + 0.08 * public_opinion;
        let prop_chance = prop_chance.min(0.98);

        let random_f64: f64 = context.rand.random();

        if random_f64 < prop_chance {
            // quality = 0.5 + 0.5 * competence + 0.08 * media_strength - 0.05 * lobbying_strength
            let quality = (0.5f64.mul_add(leg.competence, 0.5) + 0.08 * media_strength
                - 0.05 * lobbying_strength)
                .clamp(0.0, 1.0);

            // support = 0.4 + 0.3 * representativeness + 0.1 * faction_affinity + 0.08 * public_opinion
            let support = (0.3f64.mul_add(leg.representativeness, 0.4)
                + 0.1 * leg.faction_affinity
                + 0.08 * public_opinion)
                .clamp(0.0, 1.0);

            // controversy = 0.5 - 0.3 * integrity + 0.2 * (rand - 0.5) + 0.08 * partisan_polarization + 0.08 * lobbying_strength
            let raw_u64 = context.rand.next_u64();
            let noise = (raw_u64 as f64) / (u64::MAX as f64) - 0.5;
            let controversy = (0.3f64.mul_add(-leg.integrity, 0.2f64.mul_add(noise, 0.5))
                + 0.08 * partisan_polarization
                + 0.08 * lobbying_strength)
                .clamp(0.0, 1.0);

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

fn vote_in_chambers(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    context: &mut SimulationContext,
) -> (Vec<LawProposal>, bool) {
    let member_count = system.members.len() as f64;
    let mut passed = Vec::with_capacity(proposals.len());
    let mut gridlock_count = 0;
    let lobbying_strength = context.config.lobbying_strength;
    let media_strength = context.config.media_influence_strength;
    let partisan_polarization = context.config.partisan_polarization;
    let public_opinion = context.config.baseline_public_trust;
    for prop in proposals {
        let mut yes_votes = 0.0;
        // --- Coalition-building: if controversy is high, check for a coalition of similar faction_affinity ---
        let mut coalition_boost = 0.0;
        if prop.controversy > 0.6 {
            let mut max_coalition = 0;
            for i in 0..system.members.len() {
                let base_affinity = system.members[i].faction_affinity;
                let coalition_size = system
                    .members
                    .iter()
                    .filter(|l| (l.faction_affinity - base_affinity).abs() < 0.2)
                    .count();
                if coalition_size > max_coalition {
                    max_coalition = coalition_size;
                }
            }
            if max_coalition as f64 > member_count * 0.4 {
                coalition_boost = 0.12;
            }
        }
        let base_support = (0.1f64.mul_add(-prop.controversy, prop.support)
            + 0.08 * media_strength
            + 0.08 * public_opinion
            + coalition_boost)
            .clamp(0.0, 1.0);
        for leg in &system.members {
            let p = (0.2f64.mul_add(leg.faction_affinity, base_support)
                - 0.08 * partisan_polarization
                + 0.08 * lobbying_strength)
                .clamp(0.0, 1.0);
            let random_f64: f64 = context.rand.random();
            if random_f64 < p {
                yes_votes += 1.0;
            }
        }
        let filibuster_threshold = if prop.controversy > 0.6 { 0.6 } else { 0.5 };
        if yes_votes > member_count * filibuster_threshold {
            passed.push(prop.clone());
        } else {
            gridlock_count += 1;
        }
    }
    let gridlock = gridlock_count as f64 > (proposals.len() as f64 * 0.5);
    (passed, gridlock)
}

/// Simulate executive veto and legislative override.
///
/// # Veto Model
/// Each law has a non-linear probability of being vetoed by the executive, based on controversy and public opinion:
///
/// $$
/// P(\text{veto}) = \alpha \cdot \text{controversy} + \beta \cdot (1 - \text{public opinion})
/// $$
/// where $\alpha = 0.7$, $\beta = 0.3$ (tunable parameters).
///
/// # Override Model
/// Vetoed laws can be overridden by a legislative supermajority (2/3) in a re-vote:
///
/// $$
/// 	ext{Override if:}\quad \frac{\text{yes votes}}{N} > \frac{2}{3}
/// $$
///
/// Theory: U.S. Constitution, Article I, Section 7; "Presidential Vetoes and Congressional Overrides" (Rohde & Simon, 1985).
fn executive_veto(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    state: &mut SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let public_opinion = context.config.baseline_public_trust;
    let mut passed = Vec::new();
    let mut vetoed = Vec::new();
    let alpha = 0.7; // controversy weight
    let beta = 0.3; // public opinion weight

    for law in proposals {
        // Non-linear veto probability
        let p_veto = (alpha * law.controversy + beta * (1.0 - public_opinion)).clamp(0.0, 1.0);
        let roll: f64 = context.rand.random();
        if roll < p_veto {
            vetoed.push(law.clone());
        } else {
            passed.push(law.clone());
        }
    }

    // Attempt override for vetoed laws
    let override_laws = legislative_override(&vetoed, system, state, context);
    passed.extend(override_laws);
    passed
}

/// Simulate legislative override of executive veto.
///
/// # Override Threshold
/// Requires a 2/3 supermajority in a re-vote:
///
/// $$
/// 	ext{Override if:}\quad \frac{\text{yes votes}}{N} > \frac{2}{3}
/// $$
///
/// Theory: U.S. Constitution, Article I, Section 7; "Presidential Vetoes and Congressional Overrides" (Rohde & Simon, 1985).
fn legislative_override(
    vetoed: &[LawProposal],
    system: &GovernanceSystem,
    _state: &mut SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let member_count = system.members.len() as f64;
    let mut overridden = Vec::new();
    for prop in vetoed {
        let mut yes_votes = 0.0;
        // Use same voting logic as before, but require 2/3 majority
        let base_support = prop.support;
        for leg in &system.members {
            let p = (0.2f64.mul_add(leg.faction_affinity, base_support)).clamp(0.0, 1.0);
            let random_f64: f64 = context.rand.random();
            if random_f64 < p {
                yes_votes += 1.0;
            }
        }
        if yes_votes > member_count * (2.0 / 3.0) {
            overridden.push(prop.clone());
        }
    }
    overridden
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

// Pass-through stub for committee review
fn committee_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
    _context: &mut SimulationContext,
) -> Vec<LawProposal> {
    proposals.to_vec()
}

/// Debate and amendment phase for democratic lawmaking.
///
/// Debate can shift support and controversy, and amendments can nonlinearly affect law quality.
///
/// Debate/Amendment Model:
///   [
///         \text{new\_support} = \sigma(s + \Delta_{debate})
///         \text{new\_quality} = q \cdot (1 + \alpha_{amend} - \beta_{controversy})
///         \text{new\_controversy} = c + \gamma_{debate}
///   ]
/// Where $\sigma$ is a sigmoid, $\Delta_{debate}$ is a random/faction-driven shift, $\alpha_{amend}$ is amendment effect, $\beta_{controversy}$ is penalty for controversy, $\gamma_{debate}$ is debate noise.
/// Theory: Deliberative democracy (Habermas, 1996); see also "Legislative Bargaining and Amendment Dynamics" (Baron & Ferejohn, 1989).
fn debate_and_amend(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    _state: &mut SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let mut debated = Vec::with_capacity(proposals.len());
    let partisan_polarization = context.config.partisan_polarization;
    let amendment_effect = 0.08;
    let controversy_penalty = 0.12;
    for prop in proposals {
        // Debate: random/faction-driven shift in support
        let mut debate_shift = 0.0;
        for leg in &system.members {
            let alignment = 1.0 - (leg.faction_affinity - prop.support).abs();
            debate_shift += (alignment - 0.5) * (0.04 - 0.02 * partisan_polarization);
        }
        debate_shift /= system.members.len() as f64;
        // Amendment: random effect
        let amend = amendment_effect * (context.rand.random::<f64>() - 0.5);
        // Controversy penalty
        let penalty = controversy_penalty * prop.controversy;
        // Sigmoid for support
        let new_support = 1.0 / (1.0 + (-((prop.support + debate_shift) * 4.0 - 2.0)).exp());
        let new_quality = (prop.quality * (1.0 + amend - penalty)).clamp(0.0, 1.0);
        let new_controversy =
            (prop.controversy + 0.04 * (context.rand.random::<f64>() - 0.5)).clamp(0.0, 1.0);
        debated.push(LawProposal {
            quality: new_quality,
            support: new_support,
            controversy: new_controversy,
        });
    }
    debated
}
