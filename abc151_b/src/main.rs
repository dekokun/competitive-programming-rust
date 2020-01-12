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
    let N: i32 = read();
    let K: i32 = read();
    let M: i32 = read();
    let mut sum: i32 = 0;
    for _ in 0..N - 1 {
        sum += read::<i32>();
    }
    let want = std::cmp::max(M * N - sum, 0);
    if want > K {
        println!("-1");
    } else {
        println!("{}", want);
    }
}
