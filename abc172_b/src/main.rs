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
    let s: Vec<char> = read::<String>().chars().collect();
    let t: Vec<char> = read::<String>().chars().collect();
    let mut ans = 0;
    for (i, c) in s.into_iter().enumerate() {
        if c != t[i] {
            ans += 1;
        }
    }
    println!("{}", ans)
}
