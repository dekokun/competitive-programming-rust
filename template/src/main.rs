#![allow(non_snake_case, unused_macros)]

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
        n: u32,
    }
    println!("{}", solve(n));
}

fn solve(n: usize) -> usize {}
