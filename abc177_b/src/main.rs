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
    let s: String = read();
    let t: String = read();
    let s: Vec<_> = s.chars().collect();
    let t: Vec<_> = t.chars().collect();
    let mut ans = 1001;
    for i in 0..s.len() {
        if i + t.len() - 1 >= s.len() {
            break;
        }
        let mut tmp = 0;
        for (j, &c) in t.iter().enumerate() {
            if s[i + j] != c {
                tmp += 1;
            }
        }
        ans = std::cmp::min(ans, tmp);
    }
    println!("{}", ans)
}
