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
    let A: i32 = read();
    let B: i32 = read();
    let _C: i32 = read();
    let K: i32 = read();
    if A >= K {
        println!("{}", K);
        return;
    }
    if A + B >= K {
        println!("{}", A);
        return;
    }
    println!("{}", A - (K - A - B));
}
