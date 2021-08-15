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
    if s[0] != s[s.len() - 1] {
        return 1.to_string();
    }
    for cs in s.windows(2) {
        if cs[0] != s[0] && cs[1] == cs[0] {
            return 2.to_string();
        }
    }
    "-1".into()
}
