#![allow(non_snake_case)]

use std::collections::HashSet;

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
    let mut startMap = HashSet::new();
    let mut endMap = HashSet::new();
    for _ in 0..M {
        let a: usize = read();
        let b: usize = read();
        if a == 1 {
            startMap.insert(b);
        }
        if a == N {
            endMap.insert(b);
        }
        if b == 1 {
            startMap.insert(a);
        }
        if b == N {
            endMap.insert(a);
        }
    }
    if startMap.intersection(&endMap).count() != 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
