pub trait System {
    fn name(&self) -> &'static str;
    fn dependencies(&self) -> &'static [&'static str];
    fn run(&mut self, state: &mut crate::state::SimulationState, time: crate::time::SimulationTime);
}
