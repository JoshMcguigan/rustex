use std::{fmt::Display, time::{Duration, SystemTime}};

use crate::{Condition, ConditionState, EventWithTimestamp};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExpectState {
    Satisfied,
    Unsatisfied,
    Unknown,
}

pub trait Expect {
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp) -> ExpectState;
}

pub struct ContinuousExpect<D, G, T>
    where D: Display, G: Condition, T: Condition
{
    description: D,
    given: G,
    given_satisfied_time: Option<SystemTime>,
    then: T,
    state: ExpectState,
}

pub struct TriggeredExpect<D, G, W, T>
    where D: Display, G: Condition, W: Condition, T: Condition
{
    description: D,
    given: G,
    given_satisfied_time: Option<SystemTime>,
    when: W,
    then: T,
    state: ExpectState,
}

impl<D, G, T> Expect for ContinuousExpect<D, G, T>
    where D: Display, G: Condition, T: Condition
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp) -> ExpectState {
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

impl<D, G, W, T> Expect for TriggeredExpect<D, G, W, T>
    where D: Display, G: Condition, W: Condition, T: Condition
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp) -> ExpectState {
        unimplemented!()
    }
}

pub struct ExpectDescription<D> {
    description: D,
}

pub struct ExpectDescriptionGiven<D, G> {
    description: D,
    given: G,
}

pub struct ExpectDescriptionGivenWhen<D, G, W> {
    description: D,
    given: G,
    when: W,
}

pub fn verify<D>(description: D) -> ExpectDescription<D> {
    ExpectDescription {
        description,
    }
}

impl<D> ExpectDescription<D> {
    pub fn given<G>(self, given: G) -> ExpectDescriptionGiven<D, G> {
        ExpectDescriptionGiven {
            description: self.description,
            given,
        }
    }
}

impl<D, G> ExpectDescriptionGiven<D, G>
    where D: Display, G: Condition
{
    pub fn then<T>(self, then: T) -> ContinuousExpect<D, G, T>
        where T: Condition
    {
        ContinuousExpect {
            description: self.description,
            given: self.given,
            given_satisfied_time: None,
            then,
            state: ExpectState::Unknown,
        }
    }

    pub fn when<W>(self, when: W) -> ExpectDescriptionGivenWhen<D, G, W> {
        ExpectDescriptionGivenWhen {
            description: self.description,
            given: self.given,
            when,
        }
    }
}

impl<D, G, W> ExpectDescriptionGivenWhen<D, G, W>
    where D: Display, G: Condition, W: Condition
{
    pub fn then<T>(self, then: T) -> TriggeredExpect<D, G, W, T>
        where T: Condition
    {
        TriggeredExpect {
            description: self.description,
            given: self.given,
            given_satisfied_time: None,
            when: self.when,
            then,
            state: ExpectState::Unknown,
        }
    }
}
