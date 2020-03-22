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
    let mut vec: Vec<i32> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    // 最大値を取得してそれと、他の全部の和を比較し最大値が大きければ和との差が答え、最大値が小さければ0
    let &max = vec.iter().max().unwrap();
    let sum: i32 = vec.iter().sum();
    println!("{}\n{}", sum, std::cmp::max(0, max - (sum - max)));
}
