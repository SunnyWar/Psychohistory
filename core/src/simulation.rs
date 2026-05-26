use crate::config::SimulationContext;
use crate::entities::YearOutcome;
use crate::run_result::RunResult;
use sdk::Blackboard;

fn law_quality(blackboard: &Blackboard) -> f64 {
    let lobbying_pressure = blackboard.get("lobbying_pressure");
    let donor_pressure = blackboard.get("donor_pressure");
    let special_interest_degradation =
        (1.0 - 0.14 * lobbying_pressure - 0.12 * donor_pressure).clamp(0.0, 1.0);
    let law_quality = special_interest_degradation;
    blackboard.set("law_quality", law_quality);
    law_quality
}

fn corruption_level(blackboard: &Blackboard) -> f64 {
    let avg_integrity = blackboard.get("avg_integrity");
    let lobbying_pressure = blackboard.get("lobbying_pressure");
    let donor_pressure = blackboard.get("donor_pressure");
    let reelection_pressure = blackboard.get("reelection_pressure");
    let normalized_wealth_influence = blackboard.get("normalized_wealth_influence");
    let faction_formation = blackboard.get("faction_formation");
    let bad_law_drag = blackboard.get("bad_law_drag");
    let mut acc = 0.0;
    acc += (1.0 - avg_integrity) * 0.28;
    acc += lobbying_pressure * 0.24;
    acc += donor_pressure * 0.20;
    acc += reelection_pressure * 0.22;
    acc += normalized_wealth_influence * 0.14;
    acc += faction_formation * 0.10;
    acc += bad_law_drag * 0.05;
    let corruption_level = acc.clamp(0.0, 1.0);
    blackboard.set("corruption_level", corruption_level);
    corruption_level
}

fn public_trust(blackboard: &Blackboard) -> f64 {
    let prior_trust = blackboard.get("prior_trust");
    let law_quality = blackboard.get("law_quality");
    let corruption_level = blackboard.get("corruption_level");
    let bad_law_drag = blackboard.get("bad_law_drag");
    let is_gridlocked = blackboard.get("is_gridlocked");
    let external_shock = blackboard.get("external_shock");
    let media_impact = blackboard.get("media_impact");
    let gridlock_penalty = if is_gridlocked > 0.5 { 0.04 } else { 0.0 };
    let mut acc = -gridlock_penalty;
    acc += prior_trust * 0.68;
    acc += law_quality * 0.14;
    acc += corruption_level * -0.20;
    acc += bad_law_drag * -0.08;
    acc += external_shock * -0.03;
    acc += media_impact * 0.08;
    let public_trust = acc.clamp(0.0, 1.0);
    blackboard.set("public_trust", public_trust);
    public_trust
}

fn crisis_response(blackboard: &Blackboard) -> f64 {
    let legislative_competence = blackboard.get("legislative_competence");
    let judicial_competence = blackboard.get("judicial_competence");
    let avg_leadership = blackboard.get("avg_leadership");
    let expert_support_effectiveness = blackboard.get("expert_support_effectiveness");
    let policy_stock = blackboard.get("policy_stock");
    let legislative_efficiency = blackboard.get("legislative_efficiency");
    let stability_multiplier = blackboard.get("stability_multiplier");
    let mut acc = 0.0;
    acc += legislative_competence * 0.20;
    acc += judicial_competence * 0.24;
    acc += avg_leadership * 0.13;
    acc += expert_support_effectiveness * 0.16;
    acc += policy_stock * 0.11;
    acc += 0.0 * 0.22;
    let base_crisis_capability = acc;
    let crisis_response =
        (base_crisis_capability * legislative_efficiency * stability_multiplier).clamp(0.0, 1.0);
    blackboard.set("crisis_response", crisis_response);
    crisis_response
}

fn adaptability(blackboard: &Blackboard) -> f64 {
    let avg_competence = blackboard.get("avg_competence");
    let policy_stock = blackboard.get("policy_stock");
    let avg_leadership = blackboard.get("avg_leadership");
    let faction_formation = blackboard.get("faction_formation");
    let bad_law_drag = blackboard.get("bad_law_drag");
    let is_gridlocked = blackboard.get("is_gridlocked");
    let mut acc = 0.0;
    acc += avg_competence * 0.24;
    acc += policy_stock * 0.14;
    acc += avg_leadership * 0.10;
    acc += faction_formation * -0.10;
    acc += bad_law_drag * -0.08;
    acc -= if is_gridlocked > 0.5 { 0.08 } else { 0.0 };
    let adaptability = acc.clamp(0.0, 1.0);
    blackboard.set("adaptability", adaptability);
    adaptability
}

fn representation_accuracy(blackboard: &Blackboard) -> f64 {
    let avg_representation = blackboard.get("avg_representation");
    let donor_pressure = blackboard.get("donor_pressure");
    let mut acc = 0.0;
    acc += avg_representation * 0.90;
    acc += donor_pressure * -0.06;
    let representation_accuracy = acc.clamp(0.0, 1.0);
    blackboard.set("representation_accuracy", representation_accuracy);
    representation_accuracy
}

fn economic_outcome(blackboard: &Blackboard) -> f64 {
    let law_quality = blackboard.get("law_quality");
    let crisis_response = blackboard.get("crisis_response");
    let adaptability = blackboard.get("adaptability");
    let policy_stock = blackboard.get("policy_stock");
    let corruption_level = blackboard.get("corruption_level");
    let bad_law_drag = blackboard.get("bad_law_drag");
    let external_shock = blackboard.get("external_shock");
    let mut acc = 0.36;
    acc += law_quality * 0.20;
    acc += crisis_response * 0.13;
    acc += adaptability * 0.10;
    acc += policy_stock * 0.10;
    acc += corruption_level * -0.13;
    acc += bad_law_drag * -0.07;
    acc += external_shock * -0.08;
    let economic_outcome = acc.clamp(0.0, 1.0);
    blackboard.set("economic_outcome", economic_outcome);
    economic_outcome
}

pub fn simulate_year(
    blackboard: &Blackboard,
    context: &mut SimulationContext,
    _year: usize,
) -> YearOutcome {
    law_quality(blackboard);
    corruption_level(blackboard);
    public_trust(blackboard);
    crisis_response(blackboard);
    adaptability(blackboard);
    representation_accuracy(blackboard);
    economic_outcome(blackboard);

    let weights = &context.config.weights;
    let law_quality = blackboard.get("law_quality");
    let corruption_level = blackboard.get("corruption_level");
    let public_trust = blackboard.get("public_trust");
    let crisis_response = blackboard.get("crisis_response");
    let adaptability = blackboard.get("adaptability");
    let representation_accuracy = blackboard.get("representation_accuracy");
    let economic_outcome = blackboard.get("economic_outcome");

    let mut acc = 0.0;
    acc = law_quality.mul_add(weights[0], acc);
    acc = (1.0 - corruption_level).mul_add(weights[1], acc);
    acc = public_trust.mul_add(weights[2], acc);
    acc = crisis_response.mul_add(weights[3], acc);
    acc = adaptability.mul_add(weights[4], acc);
    acc = representation_accuracy.mul_add(weights[5], acc);
    acc = economic_outcome.mul_add(weights[6], acc);

    let weight_total: f64 = weights.iter().sum();
    let composite_score = if weight_total <= 0.0 {
        0.0
    } else {
        acc / weight_total
    };

    YearOutcome {
        law_quality,
        corruption_level,
        public_trust,
        crisis_response,
        adaptability,
        representation_accuracy,
        legislative_speed: 0.0,
        economic_outcome,
        composite_score,
    }
}

pub fn run_simulation(years: usize, context: &mut SimulationContext) -> RunResult {
    let blackboard = Blackboard::new();
    blackboard.set("policy_stock", 1.0);
    blackboard.set("prior_trust", 0.5);
    blackboard.set("avg_integrity", 0.5);
    blackboard.set("avg_competence", 0.5);
    blackboard.set("avg_leadership", 0.5);
    blackboard.set("avg_representation", 0.5);
    blackboard.set("lobbying_pressure", 0.0);
    blackboard.set("donor_pressure", 0.0);
    blackboard.set("reelection_pressure", 0.0);
    blackboard.set("normalized_wealth_influence", 0.0);
    blackboard.set("faction_formation", 0.0);
    blackboard.set("bad_law_drag", 0.0);
    blackboard.set("is_gridlocked", 0.0);
    blackboard.set("external_shock", 0.0);
    blackboard.set("media_impact", 0.0);
    blackboard.set("legislative_competence", 0.5);
    blackboard.set("judicial_competence", 0.5);
    blackboard.set("expert_support_effectiveness", 0.5);
    blackboard.set("legislative_efficiency", 0.5);
    blackboard.set("stability_multiplier", 1.0);

    let mut outcomes = Vec::with_capacity(years);
    for year in 0..years {
        let outcome = simulate_year(&blackboard, context, year);
        outcomes.push(outcome);
        blackboard.set("prior_trust", blackboard.get("public_trust"));
    }
    RunResult::from_outcomes(outcomes)
}
// All legacy test code removed. Only the new blackboard-based implementation remains.
