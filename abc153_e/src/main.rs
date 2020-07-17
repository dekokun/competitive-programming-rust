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
    let H: usize = read();
    let N: usize = read();
    use std::collections::HashMap;
    let mut magics: HashMap<usize, usize> = HashMap::new();
    for _ in 0..N {
        let a: usize = read();
        match magics.get(&a) {
            Some(&org) => magics.insert(a, std::cmp::min(org, read())),
            None => magics.insert(a, read()),
        };
    }
    let mut dp = vec![10_usize.pow(10); H + 10];
    dp[0] = 0;
    for i in 0..H {
        let now = dp[i];
        for (attack, power) in &magics {
            let nextIndex = if i + attack > H { H } else { i + attack };
            dp[nextIndex] = std::cmp::min(dp[nextIndex], now + power);
        }
    }
    println!("{}", dp[H]);
}
