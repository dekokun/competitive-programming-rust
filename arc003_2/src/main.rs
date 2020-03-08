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
    let N: usize = read();
    let mut vec: Vec<String> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    vec.sort();
    let mut vec2: Vec<String> = vec![];
    for v in vec {
        let rev: String = v.chars().rev().collect();
        vec2.push(rev);
    }
    vec2.sort();
    let mut ans: Vec<String> = vec![];
    for v in vec2 {
        let rev: String = v.chars().rev().collect();
        ans.push(rev);
    }
    for v in ans {
        println!("{}", v);
    }
}
