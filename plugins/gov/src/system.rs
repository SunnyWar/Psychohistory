// plugins/gov/src/system.rs
use crate::plugin::GovPlugin;
use core::system::System;
use sdk::{ReadSnapshot, SimulationPlugin};

pub struct GovSystem;

impl System for GovSystem {
    fn name(&self) -> &'static str {
        "gov"
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn std::any::Any + Send + Sync>,
        time: sdk::SimulationTime,
        _key: &'static str,
    ) {
        // Dispatch directly into the parallel simulation execution block
        GovPlugin.step(snapshot, bucket, time);
    }
}
