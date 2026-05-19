// cli/src/main.rs
use core::App;
use demog::{DemogPlugin, DemogSystem};
use econ::{EconPlugin, EconSystem};
use gov::{GovPlugin, GovSystem};
use models::{DemogState, EconState, GovState};

mod util;
use util::fmt_currency;

fn main() {
    let mut app = App::new();

    // 1. Initialize data buckets via plugin builds
    app.add_plugin(&EconPlugin);
    app.add_plugin(&GovPlugin);
    app.add_plugin(&DemogPlugin);

    // 2. Register the parallel system execution drivers into the scheduler
    app.scheduler.add_system("econ", Box::new(EconSystem));
    app.scheduler.add_system("gov", Box::new(GovSystem));
    app.scheduler.add_system("demog", Box::new(DemogSystem));

    // Seed initial values for a viable simulation run if necessary
    // (e.g., population baseline so economic growth triggers correctly)

    println!("[cli] Launching parallel macro-simulation engine...");
    app.run(12);

    app.summarize_state();

    // Retrieve and verify data frames post-simulation
    let econ = app.state.get::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov = app.state.get::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);

    let demog = app.state.get::<DemogState>("demog");
    println!("Final population: {}", demog.population);
}
