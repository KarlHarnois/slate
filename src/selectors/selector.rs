use crate::states::AppState;

pub trait Selector<Result> {
    fn select(&self, state: &AppState) -> Result;
}
