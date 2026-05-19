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

    // Seed non-zero initial baseline metrics to kick off simulation equations
    app.update_state::<DemogState>("demog", |demog| {
        demog.population = 15_000_000; // Above the 10M floor required for economic growth
        demog.birth_rate = 0.012;
    });

    app.update_state::<EconState>("econ", |econ| {
        econ.gdp = 750_000_000.0;
        econ.inflation = 0.018;
    });

    app.update_state::<GovState>("gov", |gov| {
        gov.tax_rate = 0.22;
        gov.budget = 150_000_000.0;
        gov.stability = 0.85; // 85% basic starting stability index
    });

    println!("[cli] Launching parallel macro-simulation engine...");
    app.run(12);

    app.summarize_state();

    // Print final evaluations
    let econ = app.state.get::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov = app.state.get::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);

    let demog = app.state.get::<DemogState>("demog");
    println!("Final population: {}", demog.population);
}
