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
    let a: usize = read();
    let b: usize = read();
    let ans: String = if a < b {
        std::iter::repeat(a.to_string()).take(b).collect()
    } else {
        std::iter::repeat(b.to_string()).take(a).collect()
    };
    println!("{}", ans);
}
