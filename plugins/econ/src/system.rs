use crate::plugin::EconPlugin;
use core::system::System;
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;
// plugins/econ/src/system.rs
use crate::state::EconState;
use std::collections::HashMap;

pub struct EconSystem;

impl System for EconSystem {
    fn name(&self) -> &'static str {
        "econ"
    }

    fn run_system(
        &self,
        snapshot: &ReadSnapshot,
        bucket: &mut Box<dyn Any + Send + Sync>,
        time: sdk::SimulationTime,
        _key: &'static str,
    ) {
        // Dispatch directly into the parallel simulation execution block
        EconPlugin.step(snapshot, bucket, time);
    }
}

pub fn run_econ_system(
    snapshot: &ReadSnapshot,
    econ_state: &mut EconState,
    workspace: &mut HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    time: sdk::SimulationTime,
) {
    let _tax_rate = snapshot.get::<f64>("gov:tax_rate").copied().unwrap_or(0.2);
    let stability = snapshot.get::<f64>("gov:stability").copied().unwrap_or(1.0);
    let population = snapshot
        .get::<u64>("demog:population")
        .copied()
        .unwrap_or(0);

    let dt = time.delta_years();
    if population > 10_000_000 {
        let growth_potential = (population as f64) * 50.0 * 0.001 * dt;
        let stability_drag = stability.clamp(0.1, 1.0);
        let inflation_drag = 1.0 - econ_state.inflation;
        econ_state.gdp += growth_potential * stability_drag * inflation_drag;
    }
    econ_state.inflation = 0.000_000_000_01f64.mul_add(econ_state.gdp * dt, 0.015);

    workspace.insert("econ:gdp", Box::new(econ_state.gdp));
    workspace.insert("econ:inflation", Box::new(econ_state.inflation));
}
