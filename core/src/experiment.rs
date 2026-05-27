use crate::config::SimulationContext;

use crate::run_result::RunResult;
// No longer importing run_simulation; simulation is now plugin-driven and domain-blind.
use ndarray::ArrayView1;

#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub runs: Vec<RunResult>,
    pub mean: RunResult,
    pub stddev: RunResult,
    pub n: usize,
}
impl ExperimentResult {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn confidence_interval(stddev: f64, n: usize, z: f64) -> f64 {
        if n != 0 {
            z * stddev / (n as f64).sqrt()
        } else {
            0.0
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
    years: usize,
    context: &mut SimulationContext,
    runs: usize,
    seeds: Option<&[u64]>,
) -> ExperimentResult {
    // NOTE: This function must be rewritten to use the new domain-blind simulation kernel.
    // The old run_simulation logic is removed. You must implement your own experiment logic
    // using the open-blackboard and plugin-driven architecture.
    unimplemented!("run_experiment must be rewritten for the new plugin-driven simulation kernel");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SimulationContext;
    // Removed: use crate::entities::GovernanceSystem; (domain-blind refactor)
    // Domain-blind simulation: test logic must be rewritten for new plugin-driven kernel.
    // End of test
}
