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
        let sum = |f: fn(&YearOutcome) -> f64| outcomes.iter().map(f).sum::<f64>();
        RunResult {
            average_law_quality: sum(|o| o.law_quality) / n,
            average_corruption_level: sum(|o| o.corruption_level) / n,
            average_public_trust: sum(|o| o.public_trust) / n,
            average_crisis_response: sum(|o| o.crisis_response) / n,
            average_adaptability: sum(|o| o.adaptability) / n,
            average_representation_accuracy: sum(|o| o.representation_accuracy) / n,
            average_legislative_speed: sum(|o| o.legislative_speed) / n,
            average_economic_outcome: sum(|o| o.economic_outcome) / n,
            average_composite_score: sum(|o| o.composite_score) / n,
            outcomes,
        }
    }
}
