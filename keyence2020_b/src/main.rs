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
    let mut vec: Vec<(i32, i32)> = vec![];
    for _ in 0..N {
        let x: i32 = read();
        let l: i32 = read();
        vec.push((x, l));
    }
    vec.sort_by_key(|x| x.0 + x.1);
    let (x1, l1) = vec[0];
    let mut minus: usize = 0;
    let mut right = x1 + l1;
    for &(x, l) in &vec[1..] {
        if x - l >= right {
            right = x + l;
        } else {
            minus += 1;
        }
    }
    println!("{}", N - minus);
}
