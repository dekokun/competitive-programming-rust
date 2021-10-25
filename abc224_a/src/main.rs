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
        s: Chars,
    }
    println!("{}", solve(s));
}

fn solve(s: Vec<char>) -> String {
    if s[s.len() - 1] == 'r' && s[s.len() - 2] == 'e' {
        "er"
    } else {
       "ist"
    }.to_string()
}
