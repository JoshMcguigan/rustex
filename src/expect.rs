use crate::EventWithTimestamp;

mod builder;
pub use builder::verify;

mod continuous;
pub use continuous::ContinuousExpect;

mod triggered;
pub use triggered::TriggeredExpect;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExpectState {
    Satisfied,
    Unsatisfied,
    Unknown,
}

pub trait Expect<E> {
    fn process_event(&mut self, event_with_ts: &EventWithTimestamp<E>) -> ExpectState;
}
