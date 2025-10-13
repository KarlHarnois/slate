use crate::actions::Action;
use crate::states::AppState;

pub struct Quit {}

impl Action for Quit {
    fn apply(&self, state: &mut AppState) {
        state.is_running = false;
    }
}
