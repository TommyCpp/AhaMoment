### Thread Local Storage and !Send type.
Get confused by [this](https://github.com/open-telemetry/opentelemetry-rust/pull/239).

So did a little test on how thread local storage behave in async environment.

The expected behavior is `ContextGuard` holds a reference to last context in thread local storage. 

We rewrite the `drop` so that when `ContextGuard` it will reset current context in **Thread Local** to last context its stored.

But if the `ContextGuard` is `Send`. We could send it to another thread, which in return will reset the thread local storage in that thread. 

So we must mark `ContextGuard` as `!Send` so that user cannot send it to another thread.

For example
```rust
use crate::context::{Context, ContextGuard};
use tokio::prelude::*;

mod context;

#[derive(PartialEq, Eq, Debug)]
struct Value(i32);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx1 = Context::default().with_value(Value(12));
    let cx2 = Context::default().with_value(Value(11));
    let _g1 = cx1.attach(); // _g1 will hold empty context
    let _g2 = cx2.attach(); // _g2 will hold 12
    // current context value is 11
    let mut cur = Context::current();
    assert_eq!(cur.get::<Value>(), Some(&Value(11)));

    tokio::spawn(async move {
        drop(_g2); // reset current thread context to 12
        println!("test async")
    }).await;
    // But in this thread, context is still 11
    cur = Context::current();
    let res = cur.get::<Value>();
    assert_eq!(res, Some(&Value(11)));
    Ok(())
}
```