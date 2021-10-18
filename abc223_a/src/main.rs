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
    }
    println!("{}", solve(n));
}

fn solve(n: usize) -> String {
    if n == 0 {
        "No"
    } else if n % 100 == 0 {
        "Yes"
    } else {
        "No"
    }.into()
}
