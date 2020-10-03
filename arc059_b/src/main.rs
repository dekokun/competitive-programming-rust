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
    let s: String = read();
    for i in s.chars().enumerate().collect::<Vec<_>>().windows(2) {
        let a = i[0];
        let b = i[1];
        if a.1 == b.1 {
            println!("{} {}", a.0 + 1, b.0 + 1);
            return;
        }
}
    for i in s.chars().enumerate().collect::<Vec<_>>().windows(3) {
        let a = i[0];
        let b = i[1];
        let c = i[2];
        if a.1 == b.1 || b.1 == c.1 || a.1 == c.1 {
            println!("{} {}", a.0 + 1, c.0 + 1);
            return;
        }
    }
    println!("-1 -1")
}
