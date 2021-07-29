#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::BinaryHeap;

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
        ab: [(usize, usize); n],
    }
    println!("{}", solve(k, ab));
}

fn solve(k: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut bh: BinaryHeap<_> = ab
        .into_iter()
        .flat_map(|(a, b)| vec![b, a - b])
        .collect::<Vec<_>>()
        .into();
    let mut ans = 0;
    for _ in 0..k {
        ans += bh.pop().unwrap();
    }
    ans
}
