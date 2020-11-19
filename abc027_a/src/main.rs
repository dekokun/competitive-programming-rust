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
    let l1: usize = read();
    let l2: usize = read();
    let l3: usize = read();
    println!(
        "{}",
        if l1 == l2 {
            l3
        } else if l2 == l3 {
            l1
        } else if l3 == l1 {
            l2
        } else {
            unreachable!()
        }
    );
}
