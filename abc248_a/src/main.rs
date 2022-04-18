#![allow(non_snake_case, unused_macros, dead_code)]

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

fn solve(s: Vec<char>) -> u32 {
    for i in 0..=9 {
        if s.contains(&std::char::from_digit(i, 10).unwrap()) {
            continue;
        }
        return i;
    }
    panic!()
}
