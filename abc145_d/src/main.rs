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
    let MOD = 10_i64.pow(9) + 7;
    let X: usize = read();
    let Y: usize = read();
    let sum = X + Y;
    if sum % 6 != 0 {
        println!("0");
        return;
    }
    let A = sum / 6;
    let B = std::cmp::max(X, Y) - 3 * A;
    println!("{}", combi_mod(2 * A as i64, (A + B) as i64, MOD));
}

fn pow_mod(a: i64, b: i64, modulo: i64) -> i64 {
    if b == 0 {
        1
    } else if b % 2 == 0 {
        let d = pow_mod(a, b / 2, modulo);
        d.pow(2) % modulo
    } else {
        (a * pow_mod(a, b - 1, modulo)) % modulo
    }
}

fn combi_mod(mut n: i64, c: i64, modulo: i64) -> i64 {
    let mut u = 1;
    let mut d = 1;
    for i in 1..c + 1 {
        u = u * n % modulo;
        d = d * i % modulo;

        n -= 1;
    }
    u * pow_mod(d, modulo - 2, modulo) % modulo
}
