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
    let orig: Vec<usize> = (0..N).map(|_| read()).collect();
    let mut sorted: Vec<usize> = orig.clone();
    sorted.sort();
    let min_median = sorted[(N - 1) / 2];
    let max_median = sorted[N / 2];
    for v in orig {
        if v <= min_median {
            println!("{}", max_median);
        } else {
            println!("{}", min_median);
        }
    }
}
