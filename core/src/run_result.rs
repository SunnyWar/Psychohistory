// Domain-agnostic: Remove YearOutcome dependency. Outcomes are now Vec<Vec<f64>> or similar.

#[derive(Debug, Clone, Default)]
pub struct RunResult {
    /// Domain-agnostic vector of tracked metrics (e.g., means for each tracked variable)
    pub averages: Vec<f64>,
    /// Optional: time series of tracked metrics per tick
    pub outcomes: Vec<Vec<f64>>,
}

impl RunResult {
    #[must_use]
    pub fn from_outcomes(outcomes: Vec<Vec<f64>>) -> Self {
        let n = outcomes.len() as f64;
        if n == 0.0 {
            return Self {
                averages: vec![],
                outcomes,
            };
        }
        let metric_count = outcomes[0].len();
        let mut sums = vec![0.0; metric_count];
        for outcome in &outcomes {
            for (i, value) in outcome.iter().enumerate() {
                sums[i] += value;
            }
        }
        let averages = sums.into_iter().map(|s| s / n).collect();
        Self { averages, outcomes }
    }
}
