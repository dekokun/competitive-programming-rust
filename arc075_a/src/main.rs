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
    let rems: Vec<_> = vec.iter().filter(|&x| x % 10 != 0).collect();
    let sum = vec.iter().sum::<usize>();
    let ans = if rems.is_empty() {
        0
    } else if sum % 10 != 0 {
        sum
    } else {
        let mut rems = rems;
        rems.sort();
        sum - rems[0]
    };
    println!("{}", ans);
}
