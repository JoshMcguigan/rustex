# rustex

Easily make assertions against time series data.

```rust
pub fn example_expectation() -> impl Expect {
    verify("My first continuous expect")
        .given(eq(EventTypeOneFieldOne, 10))
        .then(eq(EventTypeTwoFieldTwo, 11))
}
```
