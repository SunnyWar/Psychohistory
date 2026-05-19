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

    // 1. Load your core system plugins
    app.add_plugin(&EconPlugin);
    app.add_plugin(&GovPlugin);
    app.add_plugin(&DemogPlugin);

    // 2. Register distinct parallel runners for each country-component block
    let countries = vec!["us", "cn", "de"];

    for country in &countries {
        let econ_key = Box::leak(format!("{}:econ", country).into_boxed_str());
        let gov_key = Box::leak(format!("{}:gov", country).into_boxed_str());
        let demog_key = Box::leak(format!("{}:demog", country).into_boxed_str());

        app.state.insert(econ_key, EconState::default());
        app.state.insert(gov_key, GovState::default());
        app.state.insert(demog_key, DemogState::default());

        app.scheduler.add_system(econ_key, Box::new(EconSystem));
        app.scheduler.add_system(gov_key, Box::new(GovSystem));
        app.scheduler.add_system(demog_key, Box::new(DemogSystem));
    }

    // 3. Seed distinct geopolitical profiles using the dual-buffer update tool
    app.update_state::<EconState>("us:econ", |econ| {
        econ.gdp = 27_000_000_000_000.0; // $27 Trillion
        econ.inflation = 0.024;
    });
    app.update_state::<GovState>("us:gov", |gov| {
        gov.tax_rate = 0.21;
        gov.stability = 0.82;
    });
    app.update_state::<DemogState>("us:demog", |demog| {
        demog.population = 334_000_000;
        demog.birth_rate = 0.012;
    });

    app.update_state::<EconState>("cn:econ", |econ| {
        econ.gdp = 18_500_000_000_000.0; // $18.5 Trillion
        econ.inflation = 0.015;
    });
    app.update_state::<GovState>("cn:gov", |gov| {
        gov.tax_rate = 0.28;
        gov.stability = 0.88;
    });
    app.update_state::<DemogState>("cn:demog", |demog| {
        demog.population = 1_412_000_000;
        demog.birth_rate = 0.010;
    });

    app.update_state::<EconState>("de:econ", |econ| {
        econ.gdp = 4_200_000_000_000.0; // $4.2 Trillion
        econ.inflation = 0.017;
    });
    app.update_state::<GovState>("de:gov", |gov| {
        gov.tax_rate = 0.30;
        gov.stability = 0.90;
    });
    app.update_state::<DemogState>("de:demog", |demog| {
        demog.population = 83_000_000;
        demog.birth_rate = 0.009;
    });

    println!("[cli] Launching parallel macro-simulation engine...");
    app.run(12, sdk::TimeGranularity::Yearly);

    app.summarize_state();

    // Print final evaluations
    let econ = app.state.get::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov = app.state.get::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);

    let demog = app.state.get::<DemogState>("demog");
    println!("Final population: {}", demog.population);
}
