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
    let s: String = read();
    let t: String = read();
    let lcm = lcm(n, m);
    use std::collections::HashSet;
    let ss = HashSet::new();
    let ts = HashSet::new();
}

fn lcm(a: usize, b: usize) -> u64 {
    let gcd = gcd(a, b);
    let a = a as u64;
    let b = b as u64;
    (a * b) / gcd as u64
}
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
