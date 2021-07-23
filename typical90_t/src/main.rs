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
        a: u64,
        b: u64,
        c: u64,
    }
    println!("{}", solve(a, b, c));
}

fn solve(a: u64, b: u64, c: u64) -> String {
    if a < c.pow(b as u32) { "Yes" } else { "No" }.into()
}
