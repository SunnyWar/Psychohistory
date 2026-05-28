use sdk::{Blackboard, SimulationPlugin};
// use crate::run_result::RunResult;
use crate::state::SimulationState;
use std::sync::Arc;

/// Result of a Monte Carlo experiment: mean and stddev for each tracked metric at each step.
#[derive(Debug, Clone)]
pub struct ExperimentResult {
    /// Per-run, per-step, per-metric values: [run][step][metric]
    pub all_values: Vec<Vec<Vec<f64>>>,
    /// Mean per step per metric: [step][metric]
    pub mean: Vec<Vec<f64>>,
    /// Stddev per step per metric: [step][metric]
    pub stddev: Vec<Vec<f64>>,
    /// Number of runs
    pub n: usize,
    /// Number of steps
    pub steps: usize,
    /// Tracked metric keys
    pub tracked_keys: Vec<String>,
}

/// Compute mean and stddev for a vector of vectors (runs x metrics)
fn mean_stddev(samples: &[Vec<f64>]) -> (Vec<f64>, Vec<f64>) {
    if samples.is_empty() {
        return (vec![], vec![]);
    }
    let n = samples.len() as f64;
    let m = samples[0].len();
    let mut mean = vec![0.0; m];
    for row in samples {
        for (i, v) in row.iter().enumerate() {
            mean[i] += v;
        }
    }
    for v in &mut mean {
        *v /= n;
    }
    let mut stddev = vec![0.0; m];
    for row in samples {
        for (i, v) in row.iter().enumerate() {
            stddev[i] += (v - mean[i]).powi(2);
        }
    }
    for v in &mut stddev {
        *v = (*v / n).sqrt();
    }
    (mean, stddev)
}

/// Generic, domain-blind Monte Carlo experiment runner.
///
/// - `plugins`: slice of SimulationPlugin trait objects
/// - `ticks`: number of time steps per run
/// - `runs`: number of Monte Carlo runs
/// - `delta_t`: time step increment (f64)
/// - `tracked_keys`: slice of blackboard keys to track
/// - `state_factory`: closure to create a fresh SimulationState for each run
pub fn run_experiment(
    plugins: &[Arc<dyn SimulationPlugin>],
    ticks: usize,
    runs: usize,
    delta_t: f64,
    tracked_keys: &[String],
    mut state_factory: impl FnMut() -> SimulationState,
) -> ExperimentResult {
    let mut all_values = Vec::with_capacity(runs);
    for _ in 0..runs {
        let mut state = state_factory();
        let blackboard = Blackboard::new();
        let mut run_values = Vec::with_capacity(ticks);
        let mut t: f64 = 0.0;
        for _tick in 0..ticks {
            // Optionally, set time on blackboard if needed by plugins
            blackboard.set("t", t);
            // Execute all plugins
            let snapshot = state.as_raw_map();
            let snapshot = sdk::ReadSnapshot::new(snapshot);
            for plugin in plugins {
                plugin.execute(&snapshot, &blackboard);
            }
            // Collect tracked metrics
            let mut metrics = Vec::with_capacity(tracked_keys.len());
            for key in tracked_keys {
                metrics.push(blackboard.get(key));
            }
            run_values.push(metrics);
            t += delta_t;
            state.advance_tick();
        }
        all_values.push(run_values);
    }
    // Transpose: for each step, collect all runs' values for that step
    let steps = ticks;
    let mut mean = Vec::with_capacity(steps);
    let mut stddev = Vec::with_capacity(steps);
    for step in 0..steps {
        let samples: Vec<_> = all_values.iter().map(|run| run[step].clone()).collect();
        let (m, s) = mean_stddev(&samples);
        mean.push(m);
        stddev.push(s);
    }
    ExperimentResult {
        all_values,
        mean,
        stddev,
        n: runs,
        steps,
        tracked_keys: tracked_keys.to_vec(),
    }
}
