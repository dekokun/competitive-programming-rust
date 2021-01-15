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
    println!("{}", solve(read(), read(), read()));
}

fn solve(a: String, b: String, c: String) -> String {
    if a.chars().last().unwrap() == b.chars().next().unwrap()  && b.chars().last().unwrap() == c.chars().next().unwrap() {
        "YES"
    } else {
        "NO"
    }.into()
}

