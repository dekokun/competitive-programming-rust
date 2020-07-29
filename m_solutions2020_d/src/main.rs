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
    let A: Vec<usize> = (0..N).map(|_| read()).collect();
    let mut pairs = vec![];
    let mut max = None;
    let mut before = A[0];
    let mut min = before;
    for &v in A.iter().skip(1) {
        // 増えてる
        if before <= v {
            max = Some(v);
        } else {
            // 減ってる
            if max.is_some() {
                pairs.push((min, max.unwrap()));
                max = None;
            }
            min = v;
        }
        before = v;
    }
    if max.is_some() {
        pairs.push((min, max.unwrap()));
    }
    let mut now = 1000;
    for (min, max) in pairs {
        let count = now / min;
        now -= count * min;
        now += count * max;
    }
    println!("{}", now)
}
