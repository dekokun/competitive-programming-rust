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
    let Y: usize = read();
    let Y = Y / 1000;
    for ten_count in 0..N + 1 {
        for five_count in 0..N + 1 {
            if ten_count + five_count > N {
                continue;
            }
            let one_count = N - ten_count - five_count;
            if ten_count * 10 + five_count * 5 + one_count == Y {
                println!("{} {} {}", ten_count, five_count, one_count);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
