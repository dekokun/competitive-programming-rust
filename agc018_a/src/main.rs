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
    let n: u64 = read();
    let k: u64 = read();
    let vec: Vec<u64> = (0..n).map(|_| read()).collect();
    let &max = vec.iter().max().unwrap();
    if max < k {
        println!("IMPOSSIBLE");
        return;
    }
    let gcd = vec.iter().fold(vec[0], |acc, &v| gcd(acc, v));
    if k % gcd == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
