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
        a: usize,
        x: usize,
        y: usize,
    }
    println!("{}", solve(n, a, x, y));
}

fn solve(n: usize, a: usize, x: usize, y: usize) -> usize {
    if n > a {
        a * x + (n - a) * y
    } else {
        n * x
    }
}
