#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

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
        m: usize,
        ab: [(usize, usize); m],
    }
    println!("{}", solve(n, m, ab));
}

fn solve(_n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut graph = HashMap::new();
    for (a, b) in ab {
        let entry = graph.entry(a).or_insert_with(|| vec![]);
        entry.push(b);
        let entry = graph.entry(b).or_insert_with(|| vec![]);
        entry.push(a);
    }
    let mut ans = 0;
    'outer: for (k, v) in graph {
        let mut count = 0;
        for i in v {
            if i < k {
                count += 1;
            }
            if count > 1 {
                continue 'outer;
            }
        }
        if count == 1 {
            ans += 1;
        }
    }
    ans
}
