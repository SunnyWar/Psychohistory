use crate::{EconState, EconSystem};
use psychohistory_core::{App, Plugin};

pub struct EconPlugin;

impl Plugin for EconPlugin {
    const NAME: &'static str = "econ";

    fn build(&self, app: &mut App) {
        app.state.insert("econ", EconState::default());
        app.scheduler.add_system(Box::new(EconSystem));
    }
}
