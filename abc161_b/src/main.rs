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
    let M: usize = read();
    let mut vec: Vec<usize> = vec![];
    let mut total = 0;
    for _ in 0..N {
        let v = read();
        vec.push(v);
        total += v;
    }
    vec.sort();
    vec.reverse();
    if vec[M - 1] as f64 >= total as f64 / (4.0 * M as f64) {
        println!("Yes");
    } else {
        println!("No");
    }
}
