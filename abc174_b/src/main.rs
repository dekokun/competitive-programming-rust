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
    let D: usize = read();
    let points: Vec<(i64, i64)> = (0..N).map(|_| (read(), read())).collect();
    let ans = points.into_iter().fold(0, |acc, (x, y)| {
        if ((x.pow(2) + y.pow(2)) as f64).sqrt() <= D as f64 {
            acc + 1
        } else {
            acc
        }
    });
    println!("{}", ans)
}
