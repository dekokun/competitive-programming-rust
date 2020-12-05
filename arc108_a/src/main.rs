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
    let s: u64 = read();
    let p: u64 = read();
    for i in 1..(((p as f64).sqrt().ceil() + 1.0) as u64) {
        if p % i != 0 {
            continue;
        }
        let n = i;
        let m = p / i;
        if n + m == s {
            println!("Yes");
            return
        }
    }
    println!("No");
}
