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
        p: [usize; n],
    }
    println!("{}", solve(p));
}

fn solve(p: Vec<usize>) -> usize {
    let max = p.iter().sum();
    let mut dp = vec![vec![false; max + 101]; p.len() + 1];
    dp[0][0] = true;
    for i in 0..p.len() {
        for j in 0..=max {
            if !dp[i][j] {
                continue;
            }
            // 正解
            dp[i + 1][j + p[i]] = true;
            // 不正解
            dp[i + 1][j] = true;
        }
    }
    let mut ans = 0;
    debug!(dp);
    for &v in &dp[p.len()] {
        if v {
            ans += 1;
        }
    }
    ans
}
