use std::time::SystemTime;

mod condition;
use condition::{Condition, ConditionState};
pub use condition::{eq};

mod event_defs;

mod expect;
pub use expect::{Expect, ExpectState, verify};

mod expectations;

pub trait Key<E, T> {
    fn check(&self, event: &E) -> Option<T>;
}

pub struct EventWithTimestamp<E> {
    event: E,
    timestamp: SystemTime,
}

impl<E> EventWithTimestamp<E> {
    pub fn new(event: E, timestamp: SystemTime) -> Self {
        EventWithTimestamp {
            event,
            timestamp,
        }
    }
}
