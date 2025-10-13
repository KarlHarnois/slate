use crate::actions::Action;
use crate::states::AppState;

pub struct NoOp;

impl Action for NoOp {
    fn apply(&self, _state: &mut AppState) {}
}
