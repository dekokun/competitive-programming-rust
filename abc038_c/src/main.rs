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
    let vec: Vec<usize> = (0..N).map(|_| read()).collect();
    let mut ans: u64 = 0;
    let mut before = 0;
    let mut consequence = 0;
    fn sum(i: u64) -> u64 {
        (1..i + 1).sum()
    }
    for v in vec {
        if v > before {
            consequence += 1;
        } else {
            ans += sum(consequence);
            consequence = 1;
        }
        before = v;
    }
    ans += sum(consequence);
    println!("{}", ans);
}
