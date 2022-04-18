#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }
    println!("{}", solve(a, b, k));
}

fn solve(mut a: usize, b: usize, k: usize) -> usize {
    for scream in 0..100 {
        if b == a {
            return scream;
        }
        if b / k <= a {
            return scream + 1;
        }
        a = a * k;
    }
    panic!()
}
