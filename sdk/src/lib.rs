// sdk/src/lib.rs
use std::any::Any;
use std::collections::HashMap;

pub struct ReadSnapshot<'a> {
    inner: &'a HashMap<&'static str, Box<dyn Any + Send + Sync>>,
}

pub trait SimulationPlugin {
    fn name(&self) -> &'static str;
    fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any + Send + Sync>);
}

impl<'a> ReadSnapshot<'a> {
    pub fn new(inner: &'a HashMap<&'static str, Box<dyn Any + Send + Sync>>) -> Self {
        Self { inner }
    }

    pub fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
        self.inner.get(key)?.downcast_ref::<T>()
    }
}