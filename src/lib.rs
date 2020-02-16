use std::time::SystemTime;

mod condition;
use condition::{Condition, ConditionState};
pub use condition::{eq};

mod expect;
pub use expect::{Expect, ExpectState, verify};

mod expectations;

pub enum Event {
    EventTypeOne(EventTypeOne),
    EventTypeTwo(EventTypeTwo),
}

pub struct EventTypeOne {
    pub field_one: u32,
    pub field_two: f32,
}


pub struct EventTypeTwo {
    pub field_one: i32,
    pub field_two: u32,
}

pub trait Key<T> {
    fn check(&self, event: &Event) -> Option<T>;
}

pub struct EventTypeOneFieldOne;

impl Key<u32> for EventTypeOneFieldOne {
    fn check(&self, event: &Event) -> Option<u32> {
        if let Event::EventTypeOne(matching_event) = event {
            return Some(matching_event.field_one)
        }
        
        None
    }
}

pub struct EventTypeOneFieldTwo;

impl Key<f32> for EventTypeOneFieldTwo {
    fn check(&self, event: &Event) -> Option<f32> {
        if let Event::EventTypeOne(matching_event) = event {
            return Some(matching_event.field_two)
        }

        None
    }
}

pub struct EventTypeTwoFieldTwo;

impl Key<u32> for EventTypeTwoFieldTwo {
    fn check(&self, event: &Event) -> Option<u32> {
        if let Event::EventTypeTwo(matching_event) = event {
            return Some(matching_event.field_two)
        }
        
        None
    }
}

pub struct EventWithTimestamp {
    event: Event,
    timestamp: SystemTime,
}

impl EventWithTimestamp {
    fn new(event: Event, timestamp: SystemTime) -> Self {
        EventWithTimestamp {
            event,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{ops::Add, time::Duration};

    fn ts(millis: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH.add(Duration::from_millis(millis))
    }

    #[test]
    fn continuous_expect() {
        let events = vec![
            EventWithTimestamp::new(
                Event::EventTypeOne(
                    EventTypeOne { field_one: 10, field_two: 1.23, }
                ),
                ts(0),
            ),
            EventWithTimestamp::new(
                Event::EventTypeTwo(
                    EventTypeTwo { field_one: -10, field_two: 11, }
                ),
                ts(1),
            ),
        ];

        let mut expectation = expectations::example_continuous_expectation();

        assert_eq!(ExpectState::Unknown, expectation.process_event(&events[0]));
        assert_eq!(ExpectState::Satisfied, expectation.process_event(&events[1]));
    }
}
