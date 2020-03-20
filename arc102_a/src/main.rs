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
    let K: usize = read();
    // Kが偶数の場合は a, b, cはK/2 の倍数
    //  その場合、K/2 + n * K の個数の3乗 + m * K の個数の3乗
    // Kが奇数の場合はa, b, cはKの倍数
    // Kの倍数の個数の3乗
    let ans = if K % 2 == 0 {
        let count_1 = N / K;
        let count_2 = (N + K / 2) / K;
        count_1.pow(3) + count_2.pow(3)
    } else {
        let count = N / K;
        count.pow(3)
    };
    println!("{}", ans);
}
