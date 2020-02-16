use std::{fmt::Display, marker::PhantomData, time::{Duration, SystemTime}};

use crate::{Condition, ConditionState, EventWithTimestamp};

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

pub struct ExpectDescription<D> {
    description: D,
}

pub struct ExpectDescriptionGiven<D, E, G>
    where G: Condition<E>
{
    description: D,
    given: G,
    marker: PhantomData<E>,
}

pub struct ExpectDescriptionGivenWhen<D, E, G, W>
    where G: Condition<E>
{
    description: D,
    given: G,
    when: W,
    marker: PhantomData<E>,
}

pub fn verify<D>(description: D) -> ExpectDescription<D> {
    ExpectDescription {
        description,
    }
}

impl<D> ExpectDescription<D> {
    pub fn given<G, E>(self, given: G) -> ExpectDescriptionGiven<D, E, G>
        where G: Condition<E>
    {
        ExpectDescriptionGiven {
            description: self.description,
            given,
            marker: PhantomData,
        }
    }
}

impl<D, E, G> ExpectDescriptionGiven<D, E, G>
    where D: Display, G: Condition<E>
{
    pub fn then<T>(self, then: T) -> ContinuousExpect<D, E, G, T>
        where T: Condition<E>
    {
        ContinuousExpect {
            description: self.description,
            given: self.given,
            given_satisfied_time: None,
            then,
            state: ExpectState::Unknown,
            marker: PhantomData,
        }
    }

    pub fn when<W>(self, when: W) -> ExpectDescriptionGivenWhen<D, E, G, W>
        where W: Condition<E>
    {
        ExpectDescriptionGivenWhen {
            description: self.description,
            given: self.given,
            when,
            marker: PhantomData,
        }
    }
}

impl<D, E, G, W> ExpectDescriptionGivenWhen<D, E, G, W>
    where D: Display, G: Condition<E>, W: Condition<E>
{
    pub fn then<T>(self, then: T) -> TriggeredExpect<D, E, G, W, T>
        where T: Condition<E>
    {
        TriggeredExpect {
            description: self.description,
            given: self.given,
            given_satisfied_time: None,
            when: self.when,
            then,
            state: ExpectState::Unknown,
            marker: PhantomData,
        }
    }
}
