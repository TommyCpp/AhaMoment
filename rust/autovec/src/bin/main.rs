extern crate autovec;

use autovec::auto_vec;

fn main() {
    let res = foo_vec(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5,0]);
    for i in res {
        println!("{}", i)
    }
}


#[auto_vec]
pub fn foo(arg1: i64, arg2: i32) -> f64 {
    return (arg1 + arg2 as i64) as f64;
}