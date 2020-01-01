use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n: usize = read();
    let w: usize = read();
    // (weight, value)
    let mut vec: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        vec.push((read(), read()));
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; n + 1];
    for i in 0..n {
        for j in 0..w + 1 {
            if j < vec[i].0 as usize {
                dp[i + 1][j] = dp[i][j];
                continue;
            }
            dp[i + 1][j] = std::cmp::max(dp[i][j - vec[i].0 as usize] + vec[i].1, dp[i][j])
        }
    }
    println!("{}", dp[n][w]);
}
