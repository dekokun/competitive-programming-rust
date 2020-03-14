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
    let a: i64 = read();
    let b: i64 = read();
    let c: i64 = read();
    if c <= a + b {
        println!("No");
    } else if 2 * (a * b + b * c + c * a) < a.pow(2) + b.pow(2) + c.pow(2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
