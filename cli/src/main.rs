use clap::{Arg, Command};
use psychohistory_core::SocialCohesionPlugin;
use psychohistory_core::experiment::run_experiment;
use psychohistory_core::sdk;
use psychohistory_core::state::SimulationState;
use std::sync::Arc;

fn main() {
    let matches = Command::new("psychohistory")
        .version("1.0")
        .about("Run generic Monte Carlo system dynamics experiments")
        .arg(
            Arg::new("ticks")
                .short('y')
                .long("ticks")
                .value_name("TICKS")
                .help("Number of time steps (ticks)")
                .default_value("10"),
        )
        .arg(
            Arg::new("runs")
                .short('r')
                .long("runs")
                .value_name("RUNS")
                .help("Number of Monte Carlo runs")
                .default_value("5"),
        )
        .arg(
            Arg::new("delta_t")
                .short('d')
                .long("delta-t")
                .value_name("DELTA_T")
                .help("Time step increment (delta t)")
                .default_value("1.0"),
        )
        .get_matches();

    let ticks: usize = matches
        .get_one::<String>("ticks")
        .unwrap()
        .parse()
        .expect("Invalid ticks");
    let runs: usize = matches
        .get_one::<String>("runs")
        .unwrap()
        .parse()
        .expect("Invalid runs");
    let delta_t: f64 = matches
        .get_one::<String>("delta_t")
        .unwrap()
        .parse()
        .expect("Invalid delta_t");

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
