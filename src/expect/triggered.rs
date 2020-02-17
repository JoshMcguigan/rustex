use std::{fmt::Display, marker::PhantomData, time::SystemTime};

use crate::{Condition, EventWithTimestamp, Expect, ExpectState};

pub struct TriggeredExpect<D, E, G, W, T>
    where D: Display, G: Condition<E>, W: Condition<E>, T: Condition<E>
{
    pub(super) description: D,
    pub(super) given: G,
    pub(super) given_satisfied_time: Option<SystemTime>,
    pub(super) when: W,
    pub(super) then: T,
    pub(super) state: ExpectState,
    pub(super) marker: PhantomData<E>,
}

impl<D, E, G, W, T> Expect<E> for TriggeredExpect<D, E, G, W, T>
    where D: Display, G: Condition<E>, W: Condition<E>, T: Condition<E>
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp<E>) -> ExpectState {
        unimplemented!()
    }
}
