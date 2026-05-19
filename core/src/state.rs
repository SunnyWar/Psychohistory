// Remove misplaced method above
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::any::Any;
use std::collections::HashMap;

type ClonerFn = Box<dyn Fn(&Box<dyn Any + Send + Sync>, Option<&mut Box<dyn Any + Send + Sync>>) -> Option<Box<dyn Any + Send + Sync>> + Send + Sync>;
type ClonerMap = HashMap<&'static str, ClonerFn>;

pub struct SimulationState {
    current: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    next: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    cloners: ClonerMap,
}

impl SimulationState {
    /// Public accessor for the cloners map (for snapshotting)
    pub fn cloners(&self) -> &ClonerMap {
        &self.cloners
    }
    pub fn new() -> Self {
        Self {
            current: HashMap::new(),
            next: HashMap::new(),
            cloners: HashMap::new(),
        }
    }

    /// Inserts a new component state. Requires `Clone + Send + Sync` to support multi-threading.
    pub fn insert<T: 'static + Clone + Send + Sync>(&mut self, key: &'static str, value: T) {
        // Prevent type mismatch for existing keys
        if let Some(existing) = self.current.get(key) && (**existing).type_id() != std::any::TypeId::of::<T>() {
            panic!(
                "Type mismatch for key '{}': tried to insert {} but existing type is {:?}",
                key,
                std::any::type_name::<T>(),
                (**existing).type_id()
            );
        }
        self.current.insert(key, Box::new(value.clone()));
        self.next.insert(key, Box::new(value));

        // Capture how to clone this concrete type T dynamically, in-place
        self.cloners.insert(
            key,
            Box::new(|src_boxed, target| {
                let src_concrete = src_boxed.downcast_ref::<T>().unwrap();
                if let Some(target_boxed) = target {
                    let dst_concrete = target_boxed.downcast_mut::<T>().unwrap();
                    dst_concrete.clone_from(src_concrete);
                    None
                } else {
                    Some(Box::new(src_concrete.clone()))
                }
            }),
        );
    }

    /// Read an immutable value from the frozen *current* tick frame.
    pub fn get<T: 'static>(&self, key: &'static str) -> &T {
        self.current
            .get(key)
            .expect("Key not found in current map")
            .downcast_ref::<T>()
            .expect("Failed to downcast to requested type")
    }

    /// Grab a raw reference to the current underlying map.
    /// This is what we will wrap inside our `ReadSnapshot`.
    pub fn as_raw_map(&self) -> &HashMap<&'static str, Box<dyn Any + Send + Sync>> {
        &self.current
    }

    /// Executes a closure over every mutable component bucket in parallel.
    /// By only taking a mutable borrow of `next` internally rather than all of `self`,
    /// it allows the caller to retain an independent read-reference to `current`.
    pub fn par_for_each_mut<F>(&mut self, f: F)
    where
        F: Fn(&'static str, &mut Box<dyn Any + Send + Sync>) + Send + Sync,
    {
        // Rust's borrow checker is smart: borrowing just `self.next` leaves
        // `self.current` completely unborrowed and accessible!
        self.next.par_iter_mut().for_each(|(&key, val)| {
            f(key, val);
        });
    }

    pub fn par_iter_next<F>(&mut self, f: F)
    where
        F: Fn((&&'static str, &mut Box<dyn Any + Send + Sync>)) + Send + Sync,
    {
        self.next.par_iter_mut().for_each(f);
    }

    /// Swaps buffers by copying changes from the `next` working space into the `current` state.
    pub fn advance_tick(&mut self) {
        // Swap the current and next maps efficiently
        std::mem::swap(&mut self.current, &mut self.next);

        // Unify retention and in-place update: retain only keys present in current, and update them in-place
        self.next.retain(|key, existing_box| {
            if let Some(val) = self.current.get(key) {
                if let Some(cloner) = self.cloners.get(key) {
                    cloner(val, Some(existing_box));
                }
                true
            } else {
                false
            }
        });

        // Insert any new keys from current that are missing in next
        for (key, val) in &self.current {
            if !self.next.contains_key(key) {
                if let Some(cloner) = self.cloners.get(key) {
                    if let Some(new_box) = cloner(val, None) {
                        self.next.insert(*key, new_box);
                    }
                }
            }
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &&'static str> {
        self.current.keys()
    }

    /// Exposes exclusive mutable access *only* to the write-plane map.
    /// This allows splitting the borrow so `current` can be read simultaneously.
    pub fn mut_workspace(&mut self) -> &mut HashMap<&'static str, Box<dyn Any + Send + Sync>> {
        &mut self.next
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}
