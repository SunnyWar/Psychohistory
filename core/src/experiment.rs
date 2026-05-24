use crate::config::SimulationContext;
use crate::entities::GovernanceSystem;
use crate::run_result::RunResult;
use crate::simulation::{SimulationPlugin, run_simulation};
use ndarray::ArrayView1;

#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub runs: Vec<RunResult>,
    pub mean: RunResult,
    pub stddev: RunResult,
    pub n: usize,
}
impl ExperimentResult {
    pub fn confidence_interval(stddev: f64, n: usize, z: f64) -> f64 {
        if n == 0 {
            0.0
        } else {
            z * stddev / (n as f64).sqrt()
        }
    }
}

fn mean_stddev(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }

    let arr = ArrayView1::from(values);
    let mean = arr.mean().unwrap_or(0.0);
    let stddev = arr.std(0.0);

    (mean, stddev)
}

/// Run multiple simulations and aggregate results (mean, stddev).
pub fn run_experiment(
    system: &GovernanceSystem,
    years: usize,
    context: &mut SimulationContext,
    plugins: &[Box<dyn SimulationPlugin>],
    runs: usize,
    seeds: Option<&[u64]>,
) -> ExperimentResult {
    let mut results = Vec::with_capacity(runs);
    for i in 0..runs {
        let seed = seeds.and_then(|s| s.get(i)).copied();
        let mut run_context = SimulationContext::new(context.config.clone(), seed);
        let result = run_simulation(system, years, &mut run_context, plugins);
        results.push(result);
    }
    // Aggregate means and stddevs for each metric
    let n = results.len();
    let metric = |f: fn(&RunResult) -> f64| results.iter().map(f).collect::<Vec<_>>();
    let (mean_law_quality, std_law_quality) = mean_stddev(&metric(|r| r.average_law_quality));
    let (mean_corruption, std_corruption) = mean_stddev(&metric(|r| r.average_corruption_level));
    let (mean_trust, std_trust) = mean_stddev(&metric(|r| r.average_public_trust));
    let (mean_crisis, std_crisis) = mean_stddev(&metric(|r| r.average_crisis_response));
    let (mean_adapt, std_adapt) = mean_stddev(&metric(|r| r.average_adaptability));
    let (mean_repr, std_repr) = mean_stddev(&metric(|r| r.average_representation_accuracy));
    let (mean_speed, std_speed) = mean_stddev(&metric(|r| r.average_legislative_speed));
    let (mean_econ, std_econ) = mean_stddev(&metric(|r| r.average_economic_outcome));
    let (mean_comp, std_comp) = mean_stddev(&metric(|r| r.average_composite_score));
    let mean = RunResult {
        average_law_quality: mean_law_quality,
        average_corruption_level: mean_corruption,
        average_public_trust: mean_trust,
        average_crisis_response: mean_crisis,
        average_adaptability: mean_adapt,
        average_representation_accuracy: mean_repr,
        average_legislative_speed: mean_speed,
        average_economic_outcome: mean_econ,
        average_composite_score: mean_comp,
        outcomes: vec![],
    };
    let stddev = RunResult {
        average_law_quality: std_law_quality,
        average_corruption_level: std_corruption,
        average_public_trust: std_trust,
        average_crisis_response: std_crisis,
        average_adaptability: std_adapt,
        average_representation_accuracy: std_repr,
        average_legislative_speed: std_speed,
        average_economic_outcome: std_econ,
        average_composite_score: std_comp,
        outcomes: vec![],
    };
    ExperimentResult {
        runs: results,
        mean,
        stddev,
        n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SimulationContext;
    use crate::entities::GovernanceSystem;
    use crate::simulation::SimulationPlugin;
    #[test]
    fn test_run_experiment_basic() {
        let system = GovernanceSystem::default();
        let mut context = SimulationContext::new(Default::default(), None);
        let plugins: Vec<Box<dyn SimulationPlugin>> = vec![];
        let runs = 5;
        let years = 10;
        let result = run_experiment(&system, years, &mut context, &plugins, runs, None);
        assert_eq!(result.runs.len(), runs);
        assert_eq!(result.n, runs);
        // Means and stddevs should be in [0, 1] or 0
        let m = &result.mean;
        let s = &result.stddev;
        for v in [
            m.average_law_quality,
            m.average_corruption_level,
            m.average_public_trust,
            m.average_crisis_response,
            m.average_adaptability,
            m.average_representation_accuracy,
            m.average_legislative_speed,
            m.average_economic_outcome,
            m.average_composite_score,
            s.average_law_quality,
            s.average_corruption_level,
            s.average_public_trust,
            s.average_crisis_response,
            s.average_adaptability,
            s.average_representation_accuracy,
            s.average_legislative_speed,
            s.average_economic_outcome,
            s.average_composite_score,
        ] {
            assert!((0.0..=1.0).contains(&v) || v == 0.0);
        }
    }
    // End of test
}
