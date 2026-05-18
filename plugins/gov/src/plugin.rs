use crate::{GovState, GovSystem};
use core::{App, Plugin};

pub struct GovPlugin;

impl Plugin for GovPlugin {
    const NAME: &'static str = "gov";

    fn build(&self, app: &mut App) {
        app.state.insert("gov", GovState::default());
        app.scheduler.add_system(Box::new(GovSystem));
    }
}
