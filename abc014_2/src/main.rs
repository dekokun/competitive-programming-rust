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
    let X: usize = read();
    let mut vec: Vec<usize> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    let str = format!("{:b}", X);
    let mut ans = 0;
    for (i, c) in str.chars().rev().enumerate() {
        match c {
            '1' => ans += vec[i],
            '0' => continue,
            _ => unimplemented!(),
        }
    }
    println!("{}", ans);
}
