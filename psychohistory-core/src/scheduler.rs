pub struct Scheduler {
    systems: Vec<Box<dyn crate::system::System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system(&mut self, sys: Box<dyn crate::system::System>) {
        self.systems.push(sys);
    }

    pub fn run(&mut self, state: &mut crate::state::SimulationState, steps: u64) {
        for step in 0..steps {
            let time = crate::time::SimulationTime { step, granularity: crate::time::TimeGranularity::Monthly };
            for sys in self.systems.iter_mut() {
                sys.run(state, time);
            }
        }
    }
}
