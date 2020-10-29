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
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut bin: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for _ in 0..n {
        let t: usize = read();
        bin.push(Reverse(t));
    }
    println!("{}", bin.pop().unwrap().0);
}
