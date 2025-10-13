use crate::actions::Action;

pub trait ActionFactory {
    fn create(&self) -> Box<dyn Action>;
}
