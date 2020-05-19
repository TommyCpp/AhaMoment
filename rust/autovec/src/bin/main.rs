extern crate autovec;

use autovec::auto_vec;

fn main() {
    let test = Test {};
    let res = foo_vec(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5,0]);
    println!("Hello World");
    for i in res {
        println!("{}", i)
    }
}


struct Test {}

impl Test {
    #[auto_vec]
    pub fn test_func<'a>(&self, i: i32, j: i32) -> i32 {
        1
    }
}

#[auto_vec]
pub fn foo(arg1: i64, arg2: i32) -> f64 {
    return (arg1 + arg2 as i64) as f64;
}

