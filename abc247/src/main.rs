#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

// https://maguro.dev/debug-macro/
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
    let mut v: VecDeque<char> = s.into();
    v.pop_back();
    v.push_front('0');
    v.into_iter().collect()
}
