use crate::actions::Action;
use crate::states::AppState;

pub struct NoOp;

impl Action for NoOp {
    fn apply(self: Box<Self>, _state: &mut AppState) {}
}
