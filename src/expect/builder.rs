use std::{fmt::Display, marker::PhantomData};

use crate::{Condition, expect::{ContinuousExpect, TriggeredExpect}, ExpectState};

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
