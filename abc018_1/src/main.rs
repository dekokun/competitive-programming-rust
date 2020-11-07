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
    let vec: Vec<usize> = (0..3).map(|_| read()).collect();
    let mut vec2: Vec<_> = vec.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    vec2.sort();
    vec2.reverse();
    let mut ans = vec![0; 3];
    for (order, (_v, i)) in vec2.into_iter().enumerate() {
        ans[i] = order + 1;
    }
    for v in ans {
        println!("{}", v);
    }
}
