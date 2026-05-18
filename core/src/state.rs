use std::any::Any;
use std::collections::HashMap;

pub struct SimulationState {
    inner: HashMap<&'static str, Box<dyn Any>>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, key: &'static str, value: T) {
        self.inner.insert(key, Box::new(value));
    }

    pub fn get<T: 'static>(&self, key: &'static str) -> &T {
        self.inner
            .get(key)
            .expect("Key not found in map")
            .downcast_ref::<T>()
            .expect("Failed to downcast to requested type")
    }

    pub fn get_mut<T: 'static>(&mut self, key: &'static str) -> &mut T {
        self.inner
            .get_mut(key)
            .expect("Key not found in map")
            .downcast_mut::<T>()
            .expect("Failed to downcast to requested type")
    }

    pub fn keys(&self) -> impl Iterator<Item = &&'static str> {
        self.inner.keys()
    }
}
