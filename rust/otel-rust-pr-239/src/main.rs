use crate::context::{Context, ContextGuard};
use tokio::prelude::*;

mod context;

#[derive(PartialEq, Eq, Debug)]
struct Value(i32);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx1 = Context::default().with_value(Value(12));
    let cx2 = Context::default().with_value(Value(11));
    let _g1 = cx1.attach();
    let _g2 = cx2.attach();
    // current context value is 11
    let mut cur = Context::current();
    assert_eq!(cur.get::<Value>(), Some(&Value(11)));

    tokio::spawn(async move {
        drop(_g2); // should reset context with current 12
        let ctx = Context::current();
        assert_eq!(ctx.get::<Value>(), Some(&Value(12)));
        println!("test async")
    }).await;
    cur = Context::current();
    let res = cur.get::<Value>();
    assert_eq!(res, Some(&Value(11)));
    Ok(())
}
