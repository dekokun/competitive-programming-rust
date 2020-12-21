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
    let r1: i64 = read();
    let c1: i64 = read();
    let r2: i64 = read();
    let c2: i64 = read();
    // r1, c1を0, 0とし、右上に向かうようにする
    // rは常に大きい
    let r = (r1.max(r2) - r1.min(r2)) as u64;
    let c = (c1.max(c2) - c1.min(c2)) as u64;
    let (r, c) = (r.max(c), r.min(c));
    if c + r == 0 {
        println!("0");
        return;
    }
}
