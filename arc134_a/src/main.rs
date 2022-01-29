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
        n: usize,
        l: u64,
        w: u64,
        a: [u64; n],
    }
    println!("{}", solve(l, w, a));
}

fn solve(l: u64, w: u64, mut a: Vec<u64>) -> u64 {
    a.sort();
    let mut ans = 0;
    if a[0] != 0 {
        ans += 1 + (a[0] - 1) / w;
    }
    ans += (l - a.last().unwrap() - 1) / w;
    for window in a.windows(2) {
        ans += (window[1] - window[0] - 1) / w;
    }
    ans
}
