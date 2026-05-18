use num_format::{Locale, ToFormattedString};
use psychohistory_core::{scheduler::Scheduler, state::SimulationState};
use psychohistory_econ::{EconState, EconSystem};

fn main() {
    let mut scheduler = Scheduler::new();
    let mut state = SimulationState::new();

    // Insert domain state
    state.insert("econ", EconState::default());

    // Register domain system
    scheduler.add_system(Box::new(EconSystem));

    scheduler.run(&mut state, 12);

    let econ = state.get_mut::<EconState>("econ");
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
