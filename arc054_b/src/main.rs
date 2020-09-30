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
    let p: f64 = read();
    let mut lb = 0.0;
    let mut ub = p;
    for _ in 0..150 {
        let t1 = (2.0 * lb + ub) / 3.0;
        let t2 = (lb + 2.0 * ub) / 3.0;
        if f(t1, p) > f(t2, p) {
            lb = t1;
        } else {
            ub = t2;
        }
    }
    println!("{}", f(lb, p));
}

fn f(x: f64, p: f64) -> f64 {
    x + p * 0.5_f64.powf(x / 1.5)
}
