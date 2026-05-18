use psychohistory_core::{scheduler::Scheduler, state::SimulationState};

fn main() {
    let mut scheduler = Scheduler::new();
    let mut state = SimulationState::new();

    // TODO: load systems from config
    println!("Psychohistory simulation starting…");

    scheduler.run(&mut state, 10);

    println!("Simulation complete.");
}
