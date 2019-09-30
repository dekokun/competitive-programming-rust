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
    let n: i32 = read();
    let k: i32 = read();
    let mut heights: Vec<i32> = vec![];
    for _ in 0..n {
        heights.push(read());
    }
    let mut dp = vec![0; n as usize];
    for i in 1..n as usize {
        dp[i] = std::i32::MAX;
        for j in 1..(k + 1) as usize {
            if i < j {
                break;
            }
            dp[i] = std::cmp::min(dp[i], dp[i - j] + (heights[i - j] - heights[i]).abs());
        }
    }
    println!("{}", dp[n as usize - 1]);
}
