use crate::config::SimulationContext;
use crate::run_result::RunResult;

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

/// Run multiple simulations and aggregate results (mean, stddev).
pub fn run_experiment(
    _years: usize,
    _context: &mut SimulationContext,
    _runs: usize,
    _seeds: Option<&[u64]>,
) -> ExperimentResult {
    // NOTE: This function must be rewritten to use the new domain-blind simulation kernel.
    // The old run_simulation logic is removed. You must implement your own experiment logic
    // using the open-blackboard and plugin-driven architecture.
    unimplemented!("run_experiment must be rewritten for the new plugin-driven simulation kernel");
}
