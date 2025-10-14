use crate::actions::Action;
use crate::states::AppState;

pub trait ActionFactory {
    fn create(&self, state: &AppState) -> Box<dyn Action>;
}

impl<A: ActionFactory> Action for A {
    fn apply(self: Box<Self>, state: &mut AppState) {
        let action = self.create(state);
        action.apply(state);
    }
}
