use crate::{state::DomainRegistry, time::SimulationTime};

pub trait System {
    fn name(&self) -> &'static str;
    fn dependencies(&self) -> &'static [&'static str];
    fn run(&mut self, state: &mut DomainRegistry, time: SimulationTime);
}
