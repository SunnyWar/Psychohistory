pub use app::App;
pub use experiment::{ExperimentResult, run_experiment};
pub use plugin::Plugin;
pub use state::SimulationState;
pub use system::System;

// core/src/lib.rs
pub mod app;
pub mod config;
pub mod entities;
pub mod experiment;
pub mod plugin;
pub mod run_result;
pub mod scheduler;
pub mod simulation;
pub mod state;
pub mod system;
