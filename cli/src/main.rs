use clap::Parser;
mod cli_args;
use psychohistory_core::SocialCohesionPlugin;
use psychohistory_core::experiment::run_experiment;
use psychohistory_core::sdk;
use psychohistory_core::state::SimulationState;
use std::sync::Arc;

fn main() {
    use cli_args::CliArgs;
    let args = CliArgs::parse();
    let ticks = args.ticks;
    let runs = args.runs;
    let delta_t = args.delta_t;

    // Instantiate the reference plugin
    let plugins: Vec<Arc<dyn sdk::SimulationPlugin>> = vec![Arc::new(SocialCohesionPlugin)];

    // State factory: create SimulationState and pre-allocate SocialCohesionState
    let state_factory = || {
        let state = SimulationState::default();
        // If SocialCohesionState is needed, insert it here (example):
        // state.insert("SocialCohesionState", SocialCohesionState::default());
        state
    };

    // Track only "systemic_stability"
    let tracked_keys = vec!["systemic_stability".to_string()];

    let result = run_experiment(&plugins, ticks, runs, delta_t, &tracked_keys, state_factory);

    // Print mean and stddev timeline
    println!(
        "\nMonte Carlo Results (mean ± stddev) for '{}':",
        tracked_keys[0]
    );
    for step in 0..result.steps {
        let mean = result.mean[step][0];
        let stddev = result.stddev[step][0];
        println!("Step {:>3}: {:>8.4} ± {:>8.4}", step, mean, stddev);
    }
}
