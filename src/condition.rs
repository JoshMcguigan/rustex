use crate::{Event, Key};

#[derive(Copy, Clone, PartialEq)]
pub enum ConditionState {
    Satisfied,
    Unsatisfied,
}

pub trait Condition {
    fn process_event(&mut self, event: &Event) -> ConditionState;
}

pub struct Eq<K, T>
    where K: Key<T>
{
    key: K,
    value: T,
    state: ConditionState,
}

impl<K, T> Condition for Eq<K, T>
    where K: Key<T>, T: PartialEq
{
    fn process_event(&mut self, event: &Event) -> ConditionState {
        if let Some(value) = self.key.check(event) {
            if self.value == value {
                self.state = ConditionState::Satisfied;
            } else {
                self.state = ConditionState::Unsatisfied;
            }
        }

        self.state
    }
}

pub fn eq<K, T>(key: K, value: T) -> Eq<K,T>
    where K: Key<T>
{
    Eq {
        key,
        value,
        state: ConditionState::Unsatisfied,
    }
}
