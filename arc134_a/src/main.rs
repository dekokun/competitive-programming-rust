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
        l: i64,
        w: i64,
        a: [i64; n],
    }
    println!("{}", solve(l, w, a));
}

fn solve(l: i64, w: i64, mut a: Vec<i64>) -> i64 {
    a.push(-w);
    a.push(l);
    a.sort();
    let mut ans = 0;
    for window in a.windows(2) {
        ans += (window[1] - window[0] - 1) / w;
    }
    ans
}
