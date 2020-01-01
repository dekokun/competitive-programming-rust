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
    let N: i64 = read();
    let A: i64 = read();
    let B: i64 = read();
    let max = std::cmp::max(A, B);
    let min = std::cmp::min(A, B);
    if (max - min) % 2 == 0 {
        println!("{}", (max - min) / 2);
        return;
    }
    if (N - max) > min - 1 {
        println!("{}", min + (max - min - 1) / 2);
    } else {
        println!("{}", N - max + 1 + (max - min - 1) / 2);
    }
}
