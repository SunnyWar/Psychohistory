use std::any::Any;
use std::collections::HashMap;

pub struct DomainRegistry {
    inner: HashMap<&'static str, Box<dyn Any>>,
}

impl DomainRegistry {
    /// Creates a new, empty `DomainRegistry`.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Inserts a value of type `T` into the registry.
    pub fn insert<T: 'static>(&mut self, key: &'static str, value: T) {
        self.inner.insert(key, Box::new(value));
    }

    /// Retrieves an immutable reference to a value of type `T`.
    ///
    /// # Panics
    /// Panics if the key does not exist or if the value cannot be downcast to `T`.
    pub fn get<T: 'static>(&self, key: &'static str) -> &T {
        self.inner
            .get(key)
            .expect("Key not found in registry")
            .downcast_ref::<T>()
            .expect("Failed to downcast to requested type")
    }

    /// Retrieves a mutable reference to a value of type `T`.
    ///
    /// # Panics
    /// Panics if the key does not exist or if the value cannot be downcast to `T`.
    pub fn get_mut<T: 'static>(&mut self, key: &'static str) -> &mut T {
        self.inner
            .get_mut(key)
            .expect("Key not found in registry")
            .downcast_mut::<T>()
            .expect("Failed to downcast to requested type")
    }
}
