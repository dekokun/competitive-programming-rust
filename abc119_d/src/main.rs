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
        a: usize,
        b: usize,
        q: usize,
        s: [u64; a],
        t: [u64; b],
        x: [u64; q],
    }
    println!("{}", solve(s, t, x));
}

fn solve(s: Vec<u64>, t: Vec<u64>, x: Vec<u64>) -> u64 {
    1
}
