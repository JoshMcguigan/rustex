# rustex

Easily make assertions against time series data.

```rust
pub fn example_continuous_expectation() -> impl Expect<Event> {
    verify("My first continuous expect")
        .given(eq(EventTypeOneFieldOne, 10))
        .then(eq(EventTypeTwoFieldTwo, 11))
}

pub fn example_triggered_expectation() -> impl Expect<Event> {
    verify("My first triggered expect")
        .given(
            eq(EventTypeOneFieldOne, 10)
            .and(eq(EventTypeOneFieldTwo, 0.46))
        )
        .when(eq(EventTypeOneFieldTwo, 1.23))
        .then(eq(EventTypeTwoFieldTwo, 11))
}
```
