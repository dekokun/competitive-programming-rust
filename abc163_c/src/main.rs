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
    let mut vec: Vec<usize> = vec![];
    for _ in 1..N {
        vec.push(read::<usize>() - 1);
    }
    let mut ans_vec: Vec<usize> = vec![0; N];
    for v in vec {
        ans_vec[v] += 1;
    }
    for v in ans_vec {
        println!("{}", v);
    }
}
