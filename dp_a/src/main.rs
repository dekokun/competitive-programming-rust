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
    let mut vec = vec![];
    let mut dp = vec![0; n as usize];
    for _ in 0..n {
        vec.push(read::<i32>());
    }
    dp[1] = (vec[0] - vec[1]).abs();
    for i in 2..n as usize {
        dp[i] = std::cmp::min(
            dp[i - 1] + (vec[i - 1] - vec[i]).abs(),
            dp[i - 2] + (vec[i - 2] - vec[i]).abs(),
        );
    }
    println!("{}", dp[n as usize - 1 ]);
}
