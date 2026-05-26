// plugins/econ/src/lib.rs
pub mod plugin;
pub mod state;
pub mod system;

pub use plugin::EconPlugin;
pub use state::EconState;
pub use system::EconSystem;
