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
    let X: i64 = read();
    let K: i64 = read();
    let D: i64 = read();
    let X = if X < 0 { -X } else { X };
    let rem = X % D;
    let div = X / D;
    if div > K {
        println!("{}", X - K * D);
        return;
    }
    if div % 2 == K % 2 {
        println!("{}", rem);
    } else {
        println!("{}", D - rem);
    }
}
