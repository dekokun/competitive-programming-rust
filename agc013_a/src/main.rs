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
    let vec: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut unique = vec![vec[0]];
    for v in vec.windows(2) {
        if v[0] != v[1] {
            unique.push(v[1]);
        }
    }
    let mut ans = 0;
    let mut seq = vec![];
    for v in unique {
        if seq.is_empty() {
            seq.push(v);
            ans += 1;
            continue;
        }
        if seq.len() == 1 {
            seq.push(v);
            continue;
        }
        let (a, b, x) = (seq[seq.len() - 2], seq[seq.len() - 1], v);
        if (a > b && b > x) || (a < b && b < x) {
            seq.push(x);
        } else {
            seq = vec![x];
            ans += 1;
        }
    }
    println!("{}", ans);
}
