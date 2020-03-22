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
    // 西暦0年からA年までのそれぞれの数を数え、o年からB年までのそれぞれの数から引く
    let div_4 = B / 4 - (A - 1) / 4;
    let div_100 = B / 100 - (A - 1) / 100;
    let div_400 = B / 400 - (A - 1) / 400;
    println!("{}", div_4 - div_100 + div_400);
}
