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
    let m: usize = read();
    let n = n % 12;
    let n: f64 = n as f64;
    let m: f64 = m as f64;
    let short_degree = n * (360.0 / 12.0) + m * (360.0 / 60.0 / 12.0);
    let long_degree = m * (360.0 / 60.0);
    let mut ans = if short_degree > long_degree {
        short_degree - long_degree
    } else {
        long_degree - short_degree
    };
    if ans > 180.0 {
        ans = 360.0 - ans;
    }
    println!("{}", ans);
}
