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
    let s: String = read();
    let t: String = read();
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i + 1][j + 1] = if s[i] == t[j] {
                std::cmp::max(std::cmp::max(dp[i][j] + 1, dp[i][j + 1]), dp[i + 1][j])
            } else {
                std::cmp::max(std::cmp::max(dp[i][j], dp[i][j + 1]), dp[i + 1][j])
            };
        }
    }
    let mut res = vec![];
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1; // DP の遷移を遡る
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            res.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }
    res.reverse();
    println!("{}", res.into_iter().collect::<String>());
}
