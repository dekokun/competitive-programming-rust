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
    let max = 1_000_000;
    let mut vec = vec![0; max + 2];
    for _ in 0..n {
        vec[read::<usize>()] += 1;
        vec[read::<usize>() + 1] -= 1;
    }
    for i in 0..max + 1 {
        if i > 0 {
            vec[i] += vec[i - 1];
        }
    }
    println!("{}", vec.into_iter().max().unwrap())
}
