//#![warn(clippy::pedantic)]
pub mod app;
pub mod config;
pub mod entities;
pub mod experiment;
// legal module removed for domain-agnostic refactor
pub mod logging;
pub mod plugin;
pub mod run_result;
pub mod scheduler;
pub mod seed_util;
pub mod simulation;
pub mod state;
pub mod system;

pub use app::App;
pub use experiment::{ExperimentResult, run_experiment};
pub use logging::init_logger;
pub use plugin::Plugin;
pub use state::SimulationState;
pub use system::System;

pub mod example_social;
pub use ::sdk;
pub use example_social::SocialCohesionPlugin;
