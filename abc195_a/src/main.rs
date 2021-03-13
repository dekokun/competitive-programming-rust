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
        m: usize,
        h: usize
    }
    println!("{}", solve(m, h));
}

fn solve(m: usize, h: usize) -> String {
    if h % m == 0 {
        "Yes"
    } else {
        "No"
    }.into()
}
