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
    let mut vec = vec![];
    let mut aoki = 0;
    for _ in 0..n {
        let a: usize = read();
        let b: usize = read();
        vec.push(b + 2 * a);
        aoki += a;
    }
    vec.sort();
    vec.reverse();
    for (i, v) in vec.into_iter().enumerate() {
        if v > aoki {
            println!("{}", i + 1);
            return
        }
        aoki -= v;
    }
    println!("{}", n);
}
