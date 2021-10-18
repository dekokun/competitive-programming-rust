#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Chars};
use std::collections::VecDeque;

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
    let ans = solve(s);
    println!("{}\n{}", ans[0], ans[1]);
}

fn solve(s: Vec<char>) -> Vec<String> {
    let mut s:VecDeque<_> = s.into_iter().collect();
    let mut all = vec![];
    for _ in 0..s.len() {
        let a = s.pop_front().unwrap();
        s.push_back(a);
        all.push(s.clone().into_iter().collect::<String>());
    }
    let mut ans = vec![];
    ans.push(all.iter().min().unwrap().to_owned());
    ans.push(all.iter().max().unwrap().to_owned());
    ans
}
