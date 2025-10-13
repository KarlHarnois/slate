use crate::states::AppState;
use crate::actions::Action;

pub trait ActionFactory {
    fn create(&self) -> Box<dyn Action>;
}

impl<A: ActionFactory> Action for A {
    fn apply(&self, state: &mut AppState) {
        let action = self.create();
        action.apply(state);
    }
}
