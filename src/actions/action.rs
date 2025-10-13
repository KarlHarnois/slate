use crate::states::AppState;

pub trait Action {
    fn apply(self: Box<Self>, state: &mut AppState);
}
