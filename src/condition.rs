use crate::Key;

use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    enum TestEvent {
        TestEventOne(TestEventOne),
        TestEventTwo(TestEventTwo),
    }

    struct TestEventOne {
        field_a: u32,
        field_b: u32,
    }

    struct TestEventTwo {
        field_a: u32,
    }

    struct Event1FieldA;
    struct Event1FieldB;
    struct Event2FieldA;

    impl Key<TestEvent, u32> for Event1FieldA {
        fn check(&self, event: &TestEvent) -> Option<u32> {
            if let TestEvent::TestEventOne(event) = event {
                return Some(event.field_a)
            }

            None
        }
    }

    impl Key<TestEvent, u32> for Event1FieldB {
        fn check(&self, event: &TestEvent) -> Option<u32> {
            if let TestEvent::TestEventOne(event) = event {
                return Some(event.field_b)
            }

            None
        }
    }

    impl Key<TestEvent, u32> for Event2FieldA {
        fn check(&self, event: &TestEvent) -> Option<u32> {
            if let TestEvent::TestEventTwo(event) = event {
                return Some(event.field_a)
            }

            None
        }
    }

    fn example_event_one() -> TestEvent {
        TestEvent::TestEventOne(TestEventOne { field_a: 5, field_b: 11 })
    }

    fn example_event_two() -> TestEvent {
        TestEvent::TestEventTwo(TestEventTwo { field_a: 1 })
    }

    #[test]
    fn equal() {
        let mut condition = eq(Event1FieldA, 5);

        assert_eq!(ConditionState::Unsatisfied, condition.process_event(&example_event_two()));
        assert_eq!(ConditionState::Satisfied, condition.process_event(&example_event_one()));
    }
}
