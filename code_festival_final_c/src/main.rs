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
    let n: u64 = read();
    for i in 10..10001 {
        if f(i) == n {
            println!("{}", i);
            return;
        }
    }
    println!("-1")
}

fn f(k: u64) -> u64 {
    let s: String = k.to_string();
    let mut sum = 0;
    for (i, c) in s.chars().rev().enumerate() {
        let v = c.to_digit(10).unwrap() as u64;
        sum += v * k.pow(i as u32);
    }
    sum
}
