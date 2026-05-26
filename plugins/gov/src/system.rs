// plugins/gov/src/system.rs
use crate::plugin::GovPlugin;
use crate::state::GovState;
use core::system::System;
use sdk::{ReadSnapshot, SimulationPlugin};
use std::any::Any;
use std::collections::HashMap;

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

        // No direct workspace mutation here; kernel is responsible for publishing gov state
    }
}

pub fn run_gov_system(
    snapshot: &ReadSnapshot,
    gov_state: &mut GovState,
    workspace: &mut HashMap<&'static str, Box<dyn Any + Send + Sync>>,
    time: sdk::SimulationTime,
) {
    let gdp = snapshot.get::<f64>("econ:gdp").copied().unwrap_or(0.0);
    let inflation = snapshot
        .get::<f64>("econ:inflation")
        .copied()
        .unwrap_or(0.0);

    let dt = time.delta_years();
    let tax_revenue = gdp * gov_state.tax_rate * dt;
    gov_state.budget += tax_revenue;

    let tax_friction = if gov_state.tax_rate > 0.25 {
        (gov_state.tax_rate - 0.25) * 0.5 * dt
    } else {
        0.0
    };
    let inflation_friction = inflation * 0.2 * dt;
    let public_spending_stimulus = (gov_state.budget / gdp).min(0.05) * dt;

    gov_state.stability =
        (gov_state.stability + public_spending_stimulus - tax_friction - inflation_friction)
            .clamp(0.0, 1.0);

    gov_state.budget -= gov_state.budget * 0.08 * dt;

    workspace.insert("gov:tax_rate", Box::new(gov_state.tax_rate));
    workspace.insert("gov:budget", Box::new(gov_state.budget));
    workspace.insert("gov:stability", Box::new(gov_state.stability));
}
