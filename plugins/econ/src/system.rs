use models::{DemogState, EconState};
use sdk::ReadSnapshot;

pub fn run_econ_system(world: &ReadSnapshot, my_state: &mut Box<dyn std::any::Any + Send + Sync>) {
    // 1. Downcast your own mutable slice directly from the passed-in block
    let econ = my_state
        .downcast_mut::<EconState>()
        .expect("Failed to downcast to EconState");

    // 2. Read safely from other states using the snapshot
    if let Some(demog) = world.get::<DemogState>("demog") {
        if demog.population > 10_000_000 {
            econ.gdp += 500_000.0;
        }
    }
}
