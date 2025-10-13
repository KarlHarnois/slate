use crate::actions::Action;
use crate::states::AppState;

pub struct StartApp;

impl Action for StartApp {
    fn apply(self: Box<Self>, state: &mut AppState) {
        state.is_running = true;
    }
}
