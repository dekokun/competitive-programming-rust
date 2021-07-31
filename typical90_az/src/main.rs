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
        a: [[usize; 6]; n]
    }
    println!("{}", solve(a));
}

fn solve(a: Vec<Vec<usize>>) -> usize {
    let MOD = 10_usize.pow(9) + 7;
    a.into_iter()
        .map(|a| a.into_iter().sum::<usize>() % MOD)
        .fold(1, |acc, x: usize| acc * x % MOD)
}
