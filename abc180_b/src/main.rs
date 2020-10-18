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
    let ans =
        (0..n)
            .map(|_| read::<i64>())
            .fold((0, 0, 0), |(manhattan, euclidian, chebyshev), v| {
                (
                    manhattan + v.abs(),
                    euclidian + v * v,
                    std::cmp::max(chebyshev, v.abs()),
                )
            });
    println!("{}", ans.0);
    println!("{}", (ans.1 as f64).sqrt());
    println!("{}", ans.2);
}
