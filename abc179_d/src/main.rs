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
    let K: usize = read();
    let mut ranges = vec![];
    for _ in 0..K {
        let l: usize = read();
        let r: usize = read();
        ranges.push((l, r));
    }
    println!("{}", solve1(N, ranges));
}

fn solve1(N: usize, ranges: Vec<(usize, usize)>) -> i64 {
    let MOD = 998244353;
    let mut dp = vec![0; N * 2 + 100];
    let mut pluses: Vec<i64> = vec![0; N * 2 + 100];
    dp[0] = 1;
    let mut plus = 0;
    for i in 0..N {
        plus += pluses[i];
        plus %= MOD;
        dp[i] += plus;
        for &(l, r) in &ranges {
            pluses[i + l] += dp[i];
            pluses[i + l] %= MOD;
            pluses[i + r + 1] -= dp[i];
            pluses[i + r + 1] %= MOD;
        }
    }
    if dp[N - 1] < 0 {
        dp[N - 1] + MOD
    } else {
        dp[N - 1]
    }
}

fn solve2(N: usize, ranges: Vec<(usize, usize)>) -> i64 {
    use std::collections::HashSet;
    let mut S: HashSet<usize> = HashSet::new();
    let MOD = 998244353;
    for (l, r) in ranges {
        for i in l..=r {
            S.insert(i);
        }
    }
    let mut dp = vec![0; N * 2 + 100];
    dp[0] = 1;
    for i in 0..N {
        for &plus in &S {
            dp[i + plus] += dp[i];
            dp[i + plus] %= MOD;
        }
    }
    dp[N - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        for i in 2..10000 {
            assert_eq!(solve1(i, vec![(1, i)]), solve1(i, vec![(1, i)]));
        }
    }
}
