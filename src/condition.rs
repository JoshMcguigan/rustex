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

    fn example_event_one(field_a: u32, field_b: u32) -> TestEvent {
        TestEvent::TestEventOne(TestEventOne { field_a, field_b })
    }

    fn example_event_two(field_a: u32) -> TestEvent {
        TestEvent::TestEventTwo(TestEventTwo { field_a })
    }

    #[test]
    fn equal() {
        let mut condition = eq(Event1FieldA, 5);

        // unsatisfied by unrelated event
        assert_eq!(ConditionState::Unsatisfied, condition.process_event(&example_event_two(1)));

        // unsatisfied by related event with incorrect value
        assert_eq!(ConditionState::Unsatisfied, condition.process_event(&example_event_one(1, 1)));

        // satisfied by related event with correct value
        assert_eq!(ConditionState::Satisfied, condition.process_event(&example_event_one(5, 1)));

        // stays satisfied when seeing unrelated event
        assert_eq!(ConditionState::Satisfied, condition.process_event(&example_event_two(1)));

        // returns to unsatisfied upon seeing related event with incorrect value
        assert_eq!(ConditionState::Unsatisfied, condition.process_event(&example_event_one(1, 1)));
    }
}
