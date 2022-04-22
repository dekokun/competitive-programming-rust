#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

use proconio::{input, marker::Usize1};

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
        a: [usize; n],
        q: usize,
        qs: [(Usize1, Usize1, usize); q]
    }
    println!(
        "{}",
        solve(a, qs)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(a: Vec<usize>, qs: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut m = HashMap::new();
    for (i, v) in a.into_iter().enumerate() {
        let e = m.entry(v).or_insert(vec![]);
        e.push(i);
    }
    let mut ans = vec![];
    for (l, r, x) in qs {
        debug!(l, r, x);
        if !m.contains_key(&x) {
            ans.push(0);
            continue;
        }
        let mut tmp = m[&x].len();
        tmp -= match m[&x].binary_search(&l) {
            Ok(a) => a,
            Err(a) => a,
        };
        tmp -= match m[&x].binary_search(&r) {
            Ok(a) => m[&x].len() - a - 1,
            Err(a) => m[&x].len() - a,
        };
        ans.push(tmp);
    }
    ans
}
