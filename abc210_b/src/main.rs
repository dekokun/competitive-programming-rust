#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Chars};

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    println!("{}", solve(s));
}

fn solve(s: Vec<char>) -> String {
    for (i, c) in s.into_iter().enumerate() {
        if c == '0' {
            continue;
        }
        return if i % 2 == 0 { "Takahashi" } else { "Aoki" }.into();
    }
    panic!()
}
