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
    let mut a: Vec<(usize, usize)> = (0..N).map(|i| (i, read())).collect();
    a.sort_by_key(|&(_a, b)| b);
    let mut vec: Vec<(usize, usize)> = vec![];
    let mut now = 0;
    let mut before = a[0].1;
    for (i, v) in a {
        if before != v {
            now += 1;
        }
        vec.push((i, now));
        before = v;
    }
    vec.sort_by_key(|&(a, _b)| a);
    for v in vec {
        println!("{}", v.1);
    }
}
