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
    println!(
        "{}",
        solve(
            (0..n).map(|_| read()).collect(),
            (0..n).map(|_| read()).collect(),
            (0..n).map(|_| read()).collect()
        )
    );
}

fn solve(mut a: Vec<u64>, mut b: Vec<u64>, mut c: Vec<u64>) -> u64 {
    let n = a.len() as u64;
    a.sort();
    b.sort();
    c.sort();
    let mut ans = 0;
    let mut a_i = 0;
    let mut c_i = 0;
    for v in b {
        while a_i < n && a[a_i as usize] < v {
            a_i += 1;
        }
        if a_i == 0 && a[a_i as usize] >= v {
            continue;
        }
        while c_i < n && c[c_i as usize] <= v {
            c_i += 1;
        }
        ans += a_i * (n - c_i);
    }
    ans
}
