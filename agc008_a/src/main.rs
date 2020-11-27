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
    let x: i32 = read();
    let y: i32 = read();
    let mut anses = vec![];
    for (x, y, tmp) in &[(x, y, 0), (-x, y, 1), (x, -y, 1), (-x, -y, 2)] {
        if x <= y {
            anses.push(y - x + tmp);
        }
    }
    println!("{}", anses.into_iter().min().unwrap())
}

