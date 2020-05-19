extern crate autovec;

use autovec::auto_vec;

#[test]
fn test_sample1() {
    #[auto_vec]
    pub fn foo(arg1: i64, arg2: i32) -> f64 {
        return (arg1 + arg2 as i64) as f64;
    }
    let res1 = foo_vec(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);
    assert_eq!(res1, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
}

#[test]
#[should_panic]
fn test_sample2() {
    #[auto_vec]
    pub fn foo(arg1: i64, arg2: i32) -> f64 {
        return (arg1 + arg2 as i64) as f64;
    }
    let res1 = foo_vec(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5, 7]);
}

#[test]
fn test_sample3() {
    #[auto_vec]
    fn yay<T: std::clone::Clone + Into<i64>>(
        a: String,
        b: Vec<i64>,
        c: T,
    ) -> u64 {
        return b[0] as u64;
    }
    let s = String::from("test");
    let res = yay_vec(vec![s.clone(), s.clone(), s.clone()], vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![55, 66, 77]);
    assert_eq!(res, vec![1 as u64, 4, 7])
}

#[test]
fn test_with_self() {
    struct Test {}

    impl Test {
        #[auto_vec]
        pub fn test_func<'a>(&self, i: i32, j: i32) -> i32 {
            j
        }
    }

    let tester = Test {};
    assert_eq!(
        tester.test_func_vec(vec![12, 12, 12], vec![15, 15, 15]),
        vec![15, 15, 15]
    )
}

#[test]
fn test_with_mut_self() {
    struct Test {
        last: i32
    }

    impl Test {
        #[auto_vec]
        pub fn test_func<'a>(&mut self, i: i32, j: i32) -> i32 {
            self.last = j;
            i
        }
    }

    let mut tester = Test {
        last: 10
    };
    assert_eq!(tester.test_func_vec(vec![12, 12, 12], vec![15, 150, 1500]),
               vec![12, 12, 12]);
    assert_eq!(tester.last, 1500)
}

#[derive(Copy, Clone)]
pub struct To64 {
    val: i64
}

impl Into<i64> for To64 {
    fn into(self) -> i64 {
        self.val
    }
}

#[test]
fn test_with_struct() {
    #[auto_vec]
    fn yay<T: std::clone::Clone + Into<i64>>(
        a: String,
        b: Vec<i64>,
        c: T,
    ) -> u64 {
        return c.into() as u64;
    }
    let s = String::from("test");
    let res = yay_vec(vec![s.clone(), s.clone(), s.clone()], vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![To64 { val: 1 }, To64 { val: 2 }, To64 { val: 3 }]);
    assert_eq!(res, vec![1, 2, 3])
}


