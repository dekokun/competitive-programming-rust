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
    let S: String = read();
    let mut before;
    let mut ans = vec![];
    for c in S.chars().rev() {
        before = if c == '?' {
            // if before == 'D' {
            //     'P'
            // } else {
            //     'D'
            // }
            'D'
        } else {
            c
        };
        ans.push(before);
    }
    ans.reverse();
    println!("{}", ans.into_iter().collect::<String>())
}
