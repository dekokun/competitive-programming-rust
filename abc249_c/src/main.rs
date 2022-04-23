#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

use itertools::Itertools;
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
        k: usize,
        s: [String; n]
    }
    println!("{}", solve(k, s));
}

fn solve(k: usize, s: Vec<String>) -> usize {
    let mut ans = 0;
    for i in 1..=s.len() {
        for ss in s.iter().combinations(i) {
            let mut sums = HashMap::new();
            for s in ss {
                for c in s.chars() {
                    let e = sums.entry(c).or_insert(0);
                    *e += 1;
                }
            }
            let mut tmp = 0;
            for (_key, v) in sums {
                if v == k {
                    tmp += 1;
                }
            }
            ans = ans.max(tmp);
        }
    }
    ans
}
