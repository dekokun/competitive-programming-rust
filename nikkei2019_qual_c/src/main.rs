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
    let mut vec: Vec<_> = vec![];
    for _ in 0..n {
        vec.push((read::<i64>(), read::<i64>()));
    }
    vec.sort_by_key(|&(a, b)| (b + a));
    let mut ans = 0;
    for (i, (a, b)) in vec.into_iter().enumerate() {
        ans += if i % 2 == 0 {
            a
        } else {
            -b
        };
    }
    println!("{}", ans);
}
