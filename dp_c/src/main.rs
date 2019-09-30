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
    let mut dp: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        if i == 0 {
            dp.push(vec![read(), read(), read()]);
            continue;
        }
        let a: usize = read();
        let b: usize = read();
        let c: usize = read();
        let vec = vec![
            std::cmp::max(dp[i - 1][1] + a, dp[i - 1][2] + a),
            std::cmp::max(dp[i - 1][0] + b, dp[i - 1][2] + b),
            std::cmp::max(dp[i - 1][0] + c, dp[i - 1][1] + c),
        ];
        dp.push(vec);
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
