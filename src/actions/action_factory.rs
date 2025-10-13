use crate::actions::Action;
use crate::states::AppState;

pub trait ActionFactory {
    fn create(&self) -> Box<dyn Action>;
}

impl<A: ActionFactory> Action for A {
    fn apply(&self, state: &mut AppState) {
        let action = self.create();
        action.apply(state);
    }
}
