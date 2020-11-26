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
    let a: u64 = read();
    let b: u64 = read();
    let x: u64 = read();
    let max = 10_u64.pow(9);
    if sum(max, a, b) <= x {
        println!("{}", max);
        return;
    }

    let mut ng = max;
    let mut ok: u64 = 0;

    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if sum(mid, a, b) <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn len(n: u64) -> u64 {
    n.to_string().len() as u64
}

fn sum(n: u64, a: u64, b: u64) -> u64 {
    a * n + b * len(n)
}
