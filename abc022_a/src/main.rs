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
    let n: usize = read();
    let s: i32 = read();
    let t: i32 = read();
    let w: i32 = read();
    let mut now = w;
    let mut ans = 0;
    if s <= now && now <= t {
        ans += 1;
    }
    for _ in 0..n - 1 {
        let a: i32 = read();
        now += a;
        if s <= now && now <= t {
            ans += 1;
        }
    }
    println!("{}", ans)
}
