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
    let n: usize = read();
    let mut i = 1;
    let mut vec_9 = vec![];
    while 9_usize.pow(i) <= n {
        vec_9.push(9_usize.pow(i));
        i += 1;
    }
    let mut vec_6 = vec![];
    i = 1;
    while 6_usize.pow(i) <= n {
        vec_6.push(6_usize.pow(i));
        i += 1;
    }
    let mut dp: Vec<usize> = vec![100_000; n + 1];
    dp[0] = 0;
    for i in 0..=n {
        for &v in &vec_9 {
            for j in 1..9 {
                if i + j * v <= n {
                    dp[i + j * v] = std::cmp::min(dp[i + j * v], dp[i] + j);
                }
            }
        }
        for &v in &vec_6 {
            for j in 1..6 {
                if i + j * v <= n {
                    dp[i + j * v] = std::cmp::min(dp[i + j * v], dp[i] + j);
                }
            }
        }
        for v in 1..6 {
            if i + v <= n {
                dp[i + v] = std::cmp::min(dp[i + v], dp[i] + v);
            }
        }
    }
    println!("{}", dp[n]);
}
