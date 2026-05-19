// core/src/lib.rs
pub mod app;
pub mod experiment;
pub mod plugin;
pub mod scheduler;
pub mod state;
pub mod system;
pub mod time;

pub use app::App;
pub use plugin::Plugin;
pub use state::SimulationState;
pub use system::System;
pub use time::SimulationTime;
