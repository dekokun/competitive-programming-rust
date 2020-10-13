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
    let (n, m): (usize, usize) = (read(), read());
    if n >= m {
        println!("0");
        return;
    }
    let mut vec: Vec<i32> = vec![];
    for _ in 0..m {
        vec.push(read());
    }
    vec.sort();
    let all = vec[vec.len() - 1] - vec[0];
    let mut diffs = vec
        .windows(2)
        .into_iter()
        .map(|a| a[1] - a[0])
        .collect::<Vec<_>>();
    diffs.sort();
    diffs.reverse();
    if diffs.is_empty() {
        println!("0");
        return;
    }
    let minus: i32 = diffs[0..n - 1].into_iter().sum();
    println!("{}", all - minus)
}
