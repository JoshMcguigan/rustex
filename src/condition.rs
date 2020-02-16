use crate::Key;

use std::marker::PhantomData;

#[derive(Copy, Clone, PartialEq)]
pub enum ConditionState {
    Satisfied,
    Unsatisfied,
}

pub trait Condition<E> {
    fn process_event(&mut self, event: &E) -> ConditionState;
}

pub struct Eq<E, K, T>
    where K: Key<E, T>
{
    key: K,
    value: T,
    state: ConditionState,
    marker: PhantomData<E>,
}

impl<E, K, T> Condition<E> for Eq<E, K, T>
    where K: Key<E, T>, T: PartialEq
{
    fn process_event(&mut self, event: &E) -> ConditionState {
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

pub fn eq<E, K, T>(key: K, value: T) -> Eq<E, K,T>
    where K: Key<E, T>
{
    Eq {
        key,
        value,
        state: ConditionState::Unsatisfied,
        marker: PhantomData,
    }
}
