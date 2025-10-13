use crate::states::AppState;

pub trait Action {
    fn apply(&self, state: &mut AppState);
}
