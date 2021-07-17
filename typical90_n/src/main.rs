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
        a: [usize; n],
        b: [usize; n]
    }
    println!("{}", solve(a, b));
}

fn solve(mut a: Vec<usize>, mut b: Vec<usize>) -> usize {
    let mut ans = 0;
    a.sort();
    b.sort();
    for (i, v) in a.into_iter().enumerate() {
        ans += v.max(b[i]) - v.min(b[i]);
    }
    ans
}
