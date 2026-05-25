use crate::config::SimulationContext;
use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;
use rand::Rng;
use rand::RngExt;

pub struct DemocracyModel;

#[derive(Clone, Debug, PartialEq)]
pub struct LawProposal {
    pub quality: f64,                         // [0,1] quality of the law
    pub support: f64,                         // [0,1] support in legislature
    pub controversy: f64,                     // [0,1] how controversial
    pub enactment_year: Option<usize>,        // Year law was enacted
    pub scheduled_review_year: Option<usize>, // Year law is scheduled for judicial review
}

impl LegalSystemModel for DemocracyModel {
    fn simulate_legislative_session(
        &self,
        system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
        context: &mut SimulationContext,
    ) -> YearOutcome {
        // --- Democratic Legislative Session Simulation ---
        let proposals = propose_laws(system, state, context);
        let reviewed = committee_review(&proposals, system, state, context);
        let debated = debate_and_amend(&reviewed, system, state, context);
        let (passed, gridlock) = vote_in_chambers(&debated, system, context);
        state.is_gridlocked = gridlock;
        state.is_gridlocked = gridlock;
        let enacted = executive_veto(&passed, system, state, context, year);

        // Queue enacted laws for delayed judicial review
        for mut law in enacted.clone() {
            law.enactment_year = Some(year);
            // Schedule review 5-40 years in the future (randomized)
            let delay = 5 + (context.rand.random::<f64>() * 35.0).floor() as usize;
            law.scheduled_review_year = Some(year + delay);
            state.pending_judicial_review.push((law, year + delay));
        }

        // Each year, review laws whose scheduled_review_year == year
        let (to_review, keep): (Vec<_>, Vec<_>) = state
            .pending_judicial_review
            .drain(..)
            .partition(|(_, scheduled_year)| *scheduled_year == year);
        state.pending_judicial_review = keep;
        let mut all_laws = enacted;
        if !to_review.is_empty() {
            let laws_for_review: Vec<_> = to_review.into_iter().map(|(law, _)| law).collect();
            let reviewed = judicial_review(&laws_for_review, system, state, context);
            // Only keep laws that survive review
            all_laws.retain(|law| reviewed.contains(law));
        }

        compute_year_outcome(&all_laws, system, state)
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
                enactment_year: None,
                scheduled_review_year: None,
            });
        }
    }

    if proposals.is_empty() {
        proposals.push(LawProposal {
            quality: 0.5,
            support: 0.5,
            controversy: 0.5,
            enactment_year: None,
            scheduled_review_year: None,
        });
    }

    proposals
}

/// Simulate chamber voting, gridlock, filibuster, and coalition-building.
///
/// # Gridlock
/// Gridlock is flagged if more than half of proposals fail to pass:
/// $$
///     ext{Gridlock} = \frac{\text{failed}}{\text{total}} > 0.5
/// $$
///
/// # Filibuster
/// If controversy is high ($c > 0.6$), a supermajority (60%) is required to pass (Senate-style filibuster):
/// $$
///     ext{Threshold} =
/// \begin{cases}
///   0.6 & c > 0.6 \\
///   0.5 & \text{otherwise}
/// \end{cases}
/// $$
///
/// # Coalition-building
/// If controversy is high, factions with similar affinity may form a coalition, boosting passage odds:
/// $$
///     ext{Coalition boost} =
/// \begin{cases}
///   0.12 & \text{largest coalition} > 40\% \\
///   0 & \text{otherwise}
/// \end{cases}
/// $$
///
/// Theory: "Legislative Gridlock and Coalition Formation" (Krehbiel, 1998); "Filibuster and Supermajority Rules" (Binder & Smith, 1997).
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
            if max_coalition as f64 > 0.4 * member_count {
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
///     ext{Override if:}\quad \frac{\text{yes votes}}{N} > \frac{2}{3}
/// $$
///
/// Theory: U.S. Constitution, Article I, Section 7; "Presidential Vetoes and Congressional Overrides" (Rohde & Simon, 1985).
fn executive_veto(
    proposals: &[LawProposal],
    system: &GovernanceSystem,
    state: &mut SimulationState,
    context: &mut SimulationContext,
    year: usize,
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
        let mut law = law.clone();
        law.enactment_year = Some(year);
        law.scheduled_review_year = None; // Will be set after passage
        if roll < p_veto {
            vetoed.push(law);
        } else {
            passed.push(law);
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
///     ext{Override if:}\quad \frac{\text{yes votes}}{N} > \frac{2}{3}
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

/// Simulate judicial review and constitutional checks.
///
/// # Judicial Review Model (Immediate & Delayed)
/// Each law is subject to review by an independent judiciary. Most laws are queued for review years/decades after enactment (see `pending_judicial_review` in `SimulationState`). Each year, only laws whose `scheduled_review_year` matches the current year are reviewed, simulating real-world judicial delay.
///
/// Probability of being struck down increases with controversy and decreases with quality and public trust:
///
/// $$
/// P(\text{strike}) = \gamma \cdot \text{controversy} + \delta \cdot (1 - \text{quality}) + \epsilon \cdot (1 - \text{public trust})
/// $$
/// where $\gamma = 0.5$, $\delta = 0.3$, $\epsilon = 0.2$ (tunable parameters).
///
/// Theory: "Constitutional Courts as Guardians of Democracy" (Stone Sweet, 2000); "Judicial Review and the Rule of Law" (Shapiro, 2002); "Judicial Review and Constitutional Politics" (Vanberg, 2005); "The Political Foundations of Judicial Independence" (Ramseyer & Rasmusen, 2001)
fn judicial_review(
    proposals: &[LawProposal],
    _system: &GovernanceSystem,
    _state: &mut SimulationState,
    context: &mut SimulationContext,
) -> Vec<LawProposal> {
    let public_trust = context.config.baseline_public_trust;
    let gamma = 0.5; // controversy weight
    let delta = 0.3; // quality penalty
    let epsilon = 0.2; // public trust penalty
    let mut upheld = Vec::new();
    for law in proposals {
        let p_strike = (gamma * law.controversy
            + delta * (1.0 - law.quality)
            + epsilon * (1.0 - public_trust))
            .clamp(0.0, 1.0);
        let roll: f64 = context.rand.random();
        if roll > p_strike {
            upheld.push(law.clone());
        }
        // else: struck down, not included
    }
    upheld
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
            enactment_year: None,
            scheduled_review_year: None,
        });
    }
    debated
}
