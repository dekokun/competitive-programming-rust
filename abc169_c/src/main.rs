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
    let A: i64 = read();
    let B: String = read();
    let vec: Vec<_> = B.split('.').collect();
    let b: usize = vec[0].parse::<usize>().unwrap() * 100 + vec[1].parse::<usize>().unwrap();
    println!("{}", A * b as i64 / 100);
}
