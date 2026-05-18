use psychohistory_core::App;
use psychohistory_econ::{EconPlugin, EconState};
use psychohistory_gov::{GovPlugin, GovState};

mod util;
use util::fmt_currency;

fn main() {
    let mut app = App::new();

    app.add_plugin(EconPlugin);
    app.add_plugin(GovPlugin);

    app.run(12);

    app.summarize_state();

    let econ = app.state.get_mut::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov = app.state.get_mut::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);
}

