use core::App;
use econ::{EconPlugin, EconState};
use gov::{GovPlugin, GovState};

mod util;
use util::fmt_currency;

fn main() {
    let mut app = App::new();

    app.add_plugin(EconPlugin);
    app.add_plugin(GovPlugin);

    app.run(12);

    app.summarize_state();

    let econ = app.state.get::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));

    let gov = app.state.get::<GovState>("gov");
    println!("Final stability: {:.2}%", gov.stability * 100.0);
}
