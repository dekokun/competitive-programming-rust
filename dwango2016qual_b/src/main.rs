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
    let K: Vec<usize> = (0..N - 1).map(|_| read()).collect();
    let mut ans = vec![];
    ans.push(K[0]);
    for k in K.windows(2) {
        let before = k[0];
        let after = k[1];
        ans.push(std::cmp::min(before, after))
    }
    ans.push(*K.last().unwrap());
    println!(
        "{}",
        ans.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
