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
    let A: usize = read();
    let B: usize = read();
    for i in 0..5000 {
        if (i as f64 * 0.08).floor() as usize == A && (i as f64 * 0.1).floor() as usize == B {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
