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
        s: String
    }
    println!("{}", solve(n, s));
}

fn solve(n: usize, s: String) -> usize {
    let mut map = vec![None; n];
    let mut last = None;
    for c in s.chars().enumerate().collect::<Vec<_>>().windows(2) {}
}
