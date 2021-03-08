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
    let n = read();

    println!("{}", solve(n, read(), (0..n).map(|_| (read(), read())).collect()));
}

fn solve(_n: usize, x: usize, l: Vec<(usize, usize)>) -> i32 {
    let mut now = 0;
    for (i, (v, p)) in l.into_iter().enumerate() {
        now += v * p;
        if now > x * 100 {
            return i as i32 + 1
        }
    }
    return -1
}
