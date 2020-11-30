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
    let x: usize = read();
    let y: usize = read();
    let ans = if b >= a {
        std::cmp::min(x + (b - a) * y, x + (b - a) * x * 2)
    } else {
        if (a - b) % 2 == 0 {
            (a - b) * x + y
        } else {
            (a - b) * x
        }
    };
    println!("{}", ans)

}
