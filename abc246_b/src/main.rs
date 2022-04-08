#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let ans = solve(a, b);
    println!("{} {}", ans.0, ans.1);
}

fn solve(a: f64, b: f64) -> (f64, f64) {
    let dist = (a.powf(2.0) + b.powf(2.0)).sqrt();
    (a / dist, b / dist)
}
