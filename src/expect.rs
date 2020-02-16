use std::{fmt::Display, marker::PhantomData, time::{Duration, SystemTime}};

use crate::{Condition, ConditionState, EventWithTimestamp};

mod builder;
pub use builder::verify;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExpectState {
    Satisfied,
    Unsatisfied,
    Unknown,
}

pub trait Expect<E> {
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp<E>) -> ExpectState;
}

pub struct ContinuousExpect<D, E, G, T>
    where D: Display, G: Condition<E>, T: Condition<E>
{
    description: D,
    given: G,
    given_satisfied_time: Option<SystemTime>,
    then: T,
    state: ExpectState,
    marker: PhantomData<E>,
}

pub struct TriggeredExpect<D, E, G, W, T>
    where D: Display, G: Condition<E>, W: Condition<E>, T: Condition<E>
{
    description: D,
    given: G,
    given_satisfied_time: Option<SystemTime>,
    when: W,
    then: T,
    state: ExpectState,
    marker: PhantomData<E>,
}

impl<D, E, G, T> Expect<E> for ContinuousExpect<D, E, G, T>
    where D: Display, G: Condition<E>, T: Condition<E>
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp<E>) -> ExpectState {
        let grace_period = Duration::from_millis(5);

        let EventWithTimestamp { event, timestamp } = event_with_ts;

        if self.given.process_event(event) == ConditionState::Satisfied
            && self.given_satisfied_time == None
        {
            // the given has just transitioned from not satisfied to satisfied
            self.given_satisfied_time = Some(*timestamp);
        }

        match (self.given_satisfied_time, self.then.process_event(event)) {
            // Both given and then are satisfied
            (Some(_), ConditionState::Satisfied) => self.state = ExpectState::Satisfied,
            // Given is satisfied, but then is not satisfied
            (Some(ts), ConditionState::Unsatisfied) => {
                let time_elapsed_since_given_became_true = timestamp.duration_since(ts)
                    .expect("events should not be out of order");

                if time_elapsed_since_given_became_true < grace_period {
                    self.state = ExpectState::Unknown;
                } else {
                    self.state = ExpectState::Unsatisfied;
                }
            },
            // If the given is not satisfied we take no action
            (None, _) => {},
        };

        self.state
    }
}

impl<D, E, G, W, T> Expect<E> for TriggeredExpect<D, E, G, W, T>
    where D: Display, G: Condition<E>, W: Condition<E>, T: Condition<E>
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp<E>) -> ExpectState {
        unimplemented!()
    }
}
