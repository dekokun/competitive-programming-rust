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
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    println!("{}", solve(n, k, a, b));
}

fn solve(_n: usize, k: usize, a: Vec<usize>, b: Vec<usize>) -> String {
    let mut diff = 0;
    for (i, v) in a.into_iter().enumerate() {
        diff += v.max(b[i]) - v.min(b[i]);
    }
    if diff > k {
        "No"
    } else if (k - diff) % 2 == 0 {
        "Yes"
    } else {
        "No"
    }
    .into()
}
