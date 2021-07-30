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
        l: usize,
    }
    println!("{}", solve(n, l));
}

fn solve(n: usize, l: usize) -> usize {
    let MOD = 10_usize.pow(9) + 7;
    let mut dp = vec![0; n + l + 100];
    dp[0] = 1;
    for i in 0..=n {
        let now = dp[i];
        dp[i + 1] += now;
        dp[i + 1] %= MOD;
        dp[i + l] += now;
        dp[i + l] %= MOD;
    }
    dp[n]
}
