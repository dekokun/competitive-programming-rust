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
    let n: i64 = read();
    let m: i64 = read();
    let t: i64 = read();
    let mut now: i64 = n;
    let mut before = 0;
    for _ in 0..m {
        let a: i64 = read();
        let b: i64 = read();
        now -= a - before;
        if now <= 0 {
            println!("No");
            return;
        }
        now = std::cmp::min(n, now + (b - a));
        before = b;
    }
    now -= t - before;
    if now <= 0 {
        println!("No");
        return;
    }
    println!("Yes")
}
