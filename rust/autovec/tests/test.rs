extern crate autovec;

use autovec::auto_vec;

#[test]
fn test_sample1() {
    #[auto_vec]
    pub fn foo(arg1: i64, arg2: i32) -> f64 {
        return (arg1 + arg2 as i64) as f64;
    }

    let res1 = foo_vec(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);
    assert_eq!(res1, vec![2, 4, 6, 8, 10]);
}
