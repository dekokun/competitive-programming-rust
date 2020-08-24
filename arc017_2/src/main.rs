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
    let N: usize = read();
    let K: usize = read();
    let mut before = 0;
    let mut count = 0;
    let mut ans = 0;
    for _ in 0..N {
        let a: usize = read();
        if a <= before {
            count = 1;
        } else {
            count += 1;
        }
        before = a;
        if count >= K {
            ans += 1;
        }
    }
    println!("{}", ans)
}
