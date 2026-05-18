use psychohistory_core::{App, Plugin};
use crate::{EconState, EconSystem};

pub struct EconPlugin;

impl Plugin for EconPlugin {
    fn build(&self, app: &mut App) {
        app.state.insert("econ", EconState::default());
        app.scheduler.add_system(Box::new(EconSystem));
    }
}
