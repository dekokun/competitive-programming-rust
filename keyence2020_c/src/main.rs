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
    let S: usize = read();
    use std::iter;
    let other_num = if S == 1 { 2 } else { S - 1 };
    let ans_vec: Vec<String> = iter::repeat(S.to_string())
        .take(K)
        .chain(iter::repeat((other_num).to_string()).take(N - K))
        .collect();
    println!("{}", ans_vec.join(" "));
}
