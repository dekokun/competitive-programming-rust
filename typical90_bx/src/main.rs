#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", solve(a));
}

fn solve(a: Vec<usize>) -> String {
    let sum = a.iter().sum();
    let a = a.repeat(2);
    let mut tmp = 0;
    let mut j = 0;
    for i in 0..a.len() {
        if i != 0 {
            tmp -= a[i - 1];
        }
        if a[i] * 10 == sum {
            return "Yes".into();
        }
        if tmp * 10 == sum {
            return "Yes".into();
        }
        loop {
            if j == a.len() {
                break;
            }
            tmp += a[j];
            j += 1;
            if tmp * 10 == sum {
                return "Yes".into();
            }
            if tmp * 10 > sum {
                break;
            }
        }
    }
    "No".into()
}
