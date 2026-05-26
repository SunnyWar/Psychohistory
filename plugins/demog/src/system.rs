use crate::plugin::DemogPlugin;
use crate::state::DemogState;
use core::System;
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;
use std::collections::HashMap;

pub struct DemogSystem;

impl System for DemogSystem {
    fn name(&self) -> &'static str {
        "demog"
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
        _key: &'static str,
    ) {
        // Forward execution to the thread-safe plugin step function
        DemogPlugin.step(snapshot, bucket, time);

        // No direct workspace mutation here; kernel is responsible for publishing demog state
    }
}

pub fn run_demog_system(
    snapshot: &ReadSnapshot,
    demog_state: &mut DemogState,
    workspace: &mut HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    time: sdk::SimulationTime,
) {
    let stability_modifier = snapshot
        .get::<f64>("gov:tax_rate")
        .copied()
        .map(|v| v.min(0.5))
        .unwrap_or(0.0);

    demog_state.birth_rate = 0.01f64.mul_add(-stability_modifier, 0.015);

    let dt = time.delta_years();
    let growth_factor = 1.0f64.mul_add(demog_state.birth_rate * dt, 1.0);
    demog_state.population = (demog_state.population as f64 * growth_factor) as u64;

    workspace.insert("demog:population", Box::new(demog_state.population));
    workspace.insert("demog:birth_rate", Box::new(demog_state.birth_rate));
}
