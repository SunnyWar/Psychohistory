use std::{any, collections::HashMap};

pub struct SimulationState {
    pub data: HashMap<&'static str, Box<dyn any::Any>>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn insert<T: 'static>(&mut self, key: &'static str, value: T) {
        self.data.insert(key, Box::new(value));
    }

    pub fn get_mut<T: 'static>(&mut self, key: &'static str) -> &mut T {
        self.data.get_mut(key)
            .unwrap()
            .downcast_mut::<T>()
            .unwrap()
    }
}
