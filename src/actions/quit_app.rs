use crate::actions::Action;
use crate::states::AppState;

pub struct QuitApp;

impl Action for QuitApp {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.is_running = false;
    }
}
