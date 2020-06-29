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
    let mut vec: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        let mut tmp = 1;
        while i * tmp <= n {
            vec[i * tmp] += 1;
            tmp += 1;
        }
    }

    let mut ans: usize = 0;
    for (i, v) in vec.iter().enumerate() {
        ans += i * v;
    }
    println!("{}", ans)
}
