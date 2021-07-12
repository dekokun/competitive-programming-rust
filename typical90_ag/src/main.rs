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
        h: usize,
        w: usize,
    }
    println!("{}", solve(h, w));
}

fn solve(h: usize, w: usize) -> usize {
    if h == 1 || w == 1 {
        return h * w;
    }
    ((h + 1) / 2) * ((w + 1) / 2)
}
