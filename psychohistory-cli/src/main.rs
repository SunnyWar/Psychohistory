use psychohistory_core::App;
use psychohistory_econ::{EconPlugin, EconState};

use num_format::{Locale, ToFormattedString};

fn main() {
    let mut app = App::new();

    app.add_plugin(EconPlugin);

    app.run(12);

    let econ = app.state.get_mut::<EconState>("econ");
    println!("Final GDP: {}", fmt_currency(econ.gdp));
}

pub fn fmt_currency(value: f64) -> String {
    let s = format!("{:.2}", value); // "1012066220.50"
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0]
        .parse::<i64>()
        .unwrap()
        .to_formatted_string(&Locale::en);

    format!("${}.{}", int_part, parts[1])
}
