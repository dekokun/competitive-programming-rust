#![allow(non_snake_case)]

use std::io::{stdin, Read};
use std::str::FromStr;
fn read_option<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok()
}
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n = read();
    println!("{}", solve(n, (0..n).map(|_| read::<String>() == "AND".to_string()).collect()));
}

fn solve(n: usize, s: Vec<bool>) -> u64 {
    // DP
    let mut dp = vec![(0, 0); n + 1];
    dp[0] = (1, 1);
    for i in 0..n {
        let before = dp[i];
        dp[i + 1] = if s[i] {
            (before.0, before.0 + before.1 + before.1)
        } else {
            (before.0 + before.0 + before.1, before.1)
        };
    }
    dp[n].0
}
