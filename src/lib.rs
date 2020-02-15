use std::{fmt::Display, time::SystemTime};

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

pub struct EventTypeTwoFieldTwo;

impl Key<u32> for EventTypeTwoFieldTwo {
    fn check(&self, event: &Event) -> Option<u32> {
        if let Event::EventTypeTwo(matching_event) = event {
            return Some(matching_event.field_two)
        }
        
        None
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ConditionState {
    Satisfied,
    Unsatisfied,
}

pub trait Condition {
    fn process_event(&mut self, event: &Event) -> ConditionState;
}

pub struct Eq<K, T>
    where K: Key<T>
{
    key: K,
    value: T,
    state: ConditionState,
}

impl<K, T> Condition for Eq<K, T>
    where K: Key<T>, T: PartialEq
{
    fn process_event(&mut self, event: &Event) -> ConditionState {
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

pub fn eq<K, T>(key: K, value: T) -> Eq<K,T>
    where K: Key<T>
{
    Eq {
        key,
        value,
        state: ConditionState::Unsatisfied,
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

impl<D, G, T> Expect for ContinuousExpect<D, G, T>
    where D: Display, G: Condition, T: Condition
{
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp) -> ExpectState {
        let grace_period_ms = 5;

        let EventWithTimestamp { event, timestamp } = event_with_ts;
        if self.given.process_event(event) == ConditionState::Satisfied {
            if self.given_satisfied_time == None {
                // the given has just transitioned from not satisfied to satisfied
                self.given_satisfied_time = Some(*timestamp);
            }
            match self.then.process_event(event) {
                ConditionState::Satisfied => self.state = ExpectState::Satisfied,
                ConditionState::Unsatisfied => {
                    if timestamp.duration_since(self.given_satisfied_time.expect("FIXME"))
                        .expect("events should not be out of order")
                        .as_millis() < grace_period_ms {
                            self.state = ExpectState::Unknown;
                    } else {
                        self.state = ExpectState::Unsatisfied;
                    }
                },
            };
        }

        self.state
    }
}

pub struct ExpectDescription<D> {
    description: D,
}

pub struct ExpectDescriptionGiven<D, G> {
    description: D,
    given: G,
}

pub fn verify<D>(description: D) -> ExpectDescription<D> {
    ExpectDescription {
        description,
    }
}

impl<D> ExpectDescription<D> {
    fn given<G>(self, given: G) -> ExpectDescriptionGiven<D, G> {
        ExpectDescriptionGiven {
            description: self.description,
            given,
        }
    }
}

impl<D, G> ExpectDescriptionGiven<D, G>
    where D: Display, G: Condition
{
    fn then<T>(self, then: T) -> ContinuousExpect<D, G, T>
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

        let mut expectation = verify("My first continuous expect")
            .given(eq(EventTypeOneFieldOne, 10))
            .then(eq(EventTypeTwoFieldTwo, 11));

        assert_eq!(ExpectState::Unknown, expectation.process_event(&events[0]));
        assert_eq!(ExpectState::Satisfied, expectation.process_event(&events[1]));
    }
}
