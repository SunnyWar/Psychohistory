use std::any::Any;
   use std::collections::HashMap;

   /// A read-only snapshot of the world state from the prior tick.
   /// Every plugin gets a reference to this to check other plugins' values safely.
   pub struct ReadSnapshot<'a> {
       inner: &'a HashMap<&'static str, Box<dyn Any>>,
   }

   impl<'a> ReadSnapshot<'a> {
       pub fn new(inner: &'a HashMap<&'static str, Box<dyn Any>>) -> Self {
           Self { inner }
       }

       /// Safely downcasts and fetches data matching a specific string token identifier.
       pub fn get<T: 'static>(&self, key: &'static str) -> Option<&T> {
           self.inner.get(key)?.downcast_ref::<T>()
       }
   }

   /// The behavioral interface required for both native and third-party plugins.
   pub trait SimulationPlugin: Send + Sync {
       /// The string identifier matching this plugin's data state storage (e.g., "econ").
       fn name(&self) -> &'static str;

       /// Advances state computation by one step.
       /// - `world`: Read-only historical snapshot of all plugins.
       /// - `my_state`: Mutable isolation layer reserved strictly for this plugin's data type.
       fn step(&self, world: &ReadSnapshot, my_state: &mut Box<dyn Any>);
   }