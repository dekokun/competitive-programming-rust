#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Usize1};

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
        ab: [(Usize1, Usize1); m],
    }
    println!("{}", solve(n, m, ab));
}

fn solve(n: usize, _m: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut ans = 0;
    'outer: for (k, v) in graph.into_iter().enumerate() {
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
