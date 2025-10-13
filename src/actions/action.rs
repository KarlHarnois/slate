use crate::state::AppState;

pub trait Action {
    fn apply(&self, state: &mut AppState);
}
