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
    println!("{}", solve(n, (0..n).map(|_| read()).collect(), (0..n).map(|_| read()).collect()));
}

fn solve(n: usize, a: Vec<i32>, b: Vec<i32>) -> String {
    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    if sum == 0 {
        "Yes".into()
    } else {
        "No".into()
    }
}
