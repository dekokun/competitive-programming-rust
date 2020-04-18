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
    use std::collections::VecDeque;
    let mut vecdeque: VecDeque<usize> = VecDeque::new();
    let n: usize = read();
    for i in 0..n {
        if i % 2 == 0 {
            vecdeque.push_back(read());
        } else {
            vecdeque.push_front(read());
        }
    }
    let vec: Vec<usize> = if n % 2 == 1 {
        vecdeque.into_iter().rev().collect()
    } else {
        vecdeque.into_iter().collect()
    };
    println!(
        "{}",
        vec.into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
