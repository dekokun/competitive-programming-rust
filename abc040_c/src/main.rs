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
    let N: usize = read();
    let mut vec: Vec<i64> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    let mut dp: Vec<u64> = vec![std::u64::MAX; N];
    dp[0] = 0;
    for i in 0..N {
        if i < N - 1 {
            dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + (vec[i] - vec[i + 1]).abs() as u64);
        }
        if i < N - 2 {
            dp[i + 2] = std::cmp::min(dp[i + 2], dp[i] + (vec[i] - vec[i + 2]).abs() as u64);
        }
    }
    println!("{}", dp[N - 1]);
}
