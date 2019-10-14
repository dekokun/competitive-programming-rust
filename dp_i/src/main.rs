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
    let mut probabilities: Vec<f64> = vec![];
    for _ in 0..N {
        probabilities.push(read());
    }
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; N + 1]; N + 1];
    dp[0][0] = 1_f64;
    // 外側のループは何枚表になったか
    for i in 0..N {
        // 内側のループは何枚目まで見るか
        for j in 0..N {
            dp[i + 1][j + 1] += dp[i][j] * probabilities[j];
            dp[i][j + 1] += dp[i][j] * (1.0 - probabilities[j]);
        }
    }
    // dbg!(dp);
    // あとはheadの数が2/1を超えてるものを全部足す
    let mut ans = 0_f64;
    for i in (N / 2) + 1..N + 1 {
        ans += dp[i][N];
    }
    println!("{}", ans);
}
