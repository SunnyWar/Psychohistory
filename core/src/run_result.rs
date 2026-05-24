use crate::entities::YearOutcome;

#[derive(Debug, Clone, Default)]
pub struct RunResult {
    pub average_law_quality: f64,
    pub average_corruption_level: f64,
    pub average_public_trust: f64,
    pub average_crisis_response: f64,
    pub average_adaptability: f64,
    pub average_representation_accuracy: f64,
    pub average_legislative_speed: f64,
    pub average_economic_outcome: f64,
    pub average_composite_score: f64,
    pub outcomes: Vec<YearOutcome>,
}

impl RunResult {
    #[must_use]
    pub fn from_outcomes(outcomes: Vec<YearOutcome>) -> Self {
        let n = outcomes.len() as f64;

        let mut sum_law_quality = 0.0;
        let mut sum_corruption_level = 0.0;
        let mut sum_public_trust = 0.0;
        let mut sum_crisis_response = 0.0;
        let mut sum_adaptability = 0.0;
        let mut sum_representation_accuracy = 0.0;
        let mut sum_legislative_speed = 0.0;
        let mut sum_economic_outcome = 0.0;
        let mut sum_composite_score = 0.0;

        for o in &outcomes {
            sum_law_quality += o.law_quality;
            sum_corruption_level += o.corruption_level;
            sum_public_trust += o.public_trust;
            sum_crisis_response += o.crisis_response;
            sum_adaptability += o.adaptability;
            sum_representation_accuracy += o.representation_accuracy;
            sum_legislative_speed += o.legislative_speed;
            sum_economic_outcome += o.economic_outcome;
            sum_composite_score += o.composite_score;
        }

        Self {
            average_law_quality: sum_law_quality / n,
            average_corruption_level: sum_corruption_level / n,
            average_public_trust: sum_public_trust / n,
            average_crisis_response: sum_crisis_response / n,
            average_adaptability: sum_adaptability / n,
            average_representation_accuracy: sum_representation_accuracy / n,
            average_legislative_speed: sum_legislative_speed / n,
            average_economic_outcome: sum_economic_outcome / n,
            average_composite_score: sum_composite_score / n,
            outcomes,
        }
    }
}
