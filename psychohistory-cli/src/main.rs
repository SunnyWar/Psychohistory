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
    println!("Final GDP: {}", econ.gdp);
}
