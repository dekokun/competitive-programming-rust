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
    let L: usize = read();

    let B: Vec<usize> = (0..L).map(|_| read()).collect();
    let sum: usize = B.iter().fold(0, |acc, v| acc ^ v);
    if sum != 0 {
        println!("-1");
        return;
    }
    let mut ans = vec![0];
    for v in B {
        ans.push(v ^ ans.last().unwrap());
    }
    ans.pop();
    for v in ans {
        println!("{}", v)
    }
}
