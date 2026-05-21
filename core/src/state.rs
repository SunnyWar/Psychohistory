use log::debug;
// core/src/state.rs
use models::{EconSystemType, GovType};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sdk::ReadSnapshot;
use std::any::Any;
use std::collections::HashMap;

type ClonerFn = Box<
    dyn Fn(
            &Box<dyn Any + Send + Sync>,
            Option<&mut Box<dyn Any + Send + Sync>>,
        ) -> Option<Box<dyn Any + Send + Sync>>
        + Send
        + Sync,
>;

type ClonerMap = HashMap<&'static str, ClonerFn>;

pub struct SimulationState {
    pub(crate) current: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    pub(crate) next: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    cloners: ClonerMap,
    pub gov_type: GovType,
    pub econ_system: EconSystemType,
}

impl SimulationState {
    /// Cross-domain coupling: get immutable reference to EconState (current plane)
    pub fn get_econ_state(&self) -> Option<&models::EconState> {
        self.current
            .get("econ")?
            .downcast_ref::<models::EconState>()
    }

    /// Cross-domain coupling: get immutable reference to GovState (current plane)
    pub fn get_gov_state(&self) -> Option<&models::GovState> {
        self.current.get("gov")?.downcast_ref::<models::GovState>()
    }

    /// Cross-domain coupling: get immutable reference to DemogState (current plane)
    pub fn get_demog_state(&self) -> Option<&models::DemogState> {
        self.current
            .get("demog")?
            .downcast_ref::<models::DemogState>()
    }

    /// Cross-domain coupling: get mutable reference to EconState (write plane)
    pub fn get_econ_state_mut(&mut self) -> Option<&mut models::EconState> {
        self.next
            .get_mut("econ")?
            .downcast_mut::<models::EconState>()
    }

    /// Cross-domain coupling: get mutable reference to GovState (write plane)
    pub fn get_gov_state_mut(&mut self) -> Option<&mut models::GovState> {
        self.next.get_mut("gov")?.downcast_mut::<models::GovState>()
    }

    /// Cross-domain coupling: get mutable reference to DemogState (write plane)
    pub fn get_demog_state_mut(&mut self) -> Option<&mut models::DemogState> {
        self.next
            .get_mut("demog")?
            .downcast_mut::<models::DemogState>()
    }
}

impl SimulationState {
    /// Change the government type mid-simulation.
    pub fn set_gov_type(&mut self, new_type: GovType) {
        debug!(
            "Changing government type: {:?} -> {:?}",
            self.gov_type, new_type
        );
        self.gov_type = new_type;
    }

    /// Change the economic system type mid-simulation.
    pub fn set_econ_system(&mut self, new_type: EconSystemType) {
        debug!(
            "Changing economic system type: {:?} -> {:?}",
            self.econ_system, new_type
        );
        self.econ_system = new_type;
    }

    /// Mutates a component state across both the current and next data planes.
    /// Essential for seeding initial baselines prior to running the simulation loop.
    pub fn update_initial_state<T: 'static>(
        &mut self,
        key: &'static str,
        mutator: &mut impl FnMut(&mut T),
    ) {
        debug!("Updating initial state for key: {key}");
        if let Some(boxed) = self.current.get_mut(key)
            && let Some(concrete) = boxed.downcast_mut::<T>()
        {
            mutator(concrete);
        }
        if let Some(boxed) = self.next.get_mut(key)
            && let Some(concrete) = boxed.downcast_mut::<T>()
        {
            mutator(concrete);
        }
    }
    /// Executes registered systems across the parallel data planes.
    /// By building the ReadSnapshot internally from `self.current`, we cleanly split
    /// the borrow from `self.next` so Rayon workers can access both simultaneously.
    pub fn par_execute_systems<F>(&mut self, f: F)
    where
        F: Fn(&ReadSnapshot, &'static str, &mut Box<dyn Any + Send + Sync>) + Send + Sync,
    {
        // 1. Create the immutable read plane snapshot.
        // Because it only wraps an immutable reference, it is safely `Sync` and shared across threads.
        let snapshot = ReadSnapshot::new(&self.current);
        let snapshot_ref = &snapshot;

        // 2. Drive mutations strictly over the disjoint mutable write-plane
        self.next.par_iter_mut().for_each(|(&key, val)| {
            f(snapshot_ref, key, val);
        });
    }

    /// Public accessor for the cloners map (for snapshotting)
    pub fn cloners(&self) -> &ClonerMap {
        &self.cloners
    }
    pub fn new() -> Self {
        Self {
            current: HashMap::new(),
            next: HashMap::new(),
            cloners: HashMap::new(),
            gov_type: GovType::Democracy,
            econ_system: EconSystemType::Market,
        }
    }

    /// Inserts a new component state. Requires `Clone + Send + Sync` to support multi-threading.
    pub fn insert<T: 'static + Clone + Send + Sync>(&mut self, key: &'static str, value: T) {
        // Prevent type mismatch for existing keys
        if let Some(existing) = self.current.get(key)
            && (**existing).type_id() != std::any::TypeId::of::<T>()
        {
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
            if let Some(val) = self.current.get(key)
                && let Some(cloner) = self.cloners.get(key)
            {
                cloner(val, Some(existing_box));
                true
            } else {
                false
            }
        });

        // Insert any new keys from current that are missing in next
        for (key, val) in &self.current {
            if !self.next.contains_key(key)
                && let Some(cloner) = self.cloners.get(key)
                && let Some(new_box) = cloner(val, None)
            {
                self.next.insert(*key, new_box);
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
