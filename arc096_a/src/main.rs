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
    let c: usize = read();
    let x: usize = read();
    let y: usize = read();
    // xの方が多いとする
    let (a, b, x, y) = if x < y { (b, a, y, x) } else { (a, b, x, y) };
    let mut ans = 0;
    ans += if a + b > 2 * c {
        2 * c * y
    } else {
        (a + b) * y
    };
    ans += if 2 * c < a {
        2 * c * (x - y)
    } else {
        a * (x - y)
    };
    println!("{}", ans)
}
