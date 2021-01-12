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
    println!("{}", solve(read()));
}

fn solve(n: u64) -> u64 {
    let mut ans = 0;
    for i in 1..=(n as f64).sqrt().ceil() as u64 {
        if n % i != 0 {
            continue;
        }
        let mut candidates = vec![];
        if n / i == i {
            candidates.push(i - 1);
        } else {
            candidates.push(i - 1);
            candidates.push((n / i) - 1);
        }
        for v in candidates {
            if v == 0 {
                continue;
            }
            if n / v == n % v {
                ans += v;
            }
        }
    }
    ans
}
