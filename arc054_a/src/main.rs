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
    let l: f64 = read();
    let x: f64 = read();
    let y: f64 = read();
    let s: f64 = read();
    let d: f64 = read();
    let goal = if d > s {
        d - s
    } else {
        l + (d - s)
    };
    let ans1 = goal / (x + y);
    let ans2 = (l - goal) / (y - x);
    dbg!(ans1, ans2);
    let ans = if ans2 < 0.0 {
        ans1
    } else if ans1 > ans2 {
        ans2
    } else {
        ans1
    };
    println!("{}", ans)
}
