#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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
    println!("{}", solve((0..n).map(|_| (read(), read())).collect()));
}

fn solve(ab: Vec<(usize, usize)>) -> usize {
    let mut ans = 10_usize.pow(9);
    for i in 0..ab.len() {
        for j in 0..ab.len() {
            let tmp = if i == j {
                ab[i].0 + ab[j].1
            } else {
                ab[i].0.max(ab[j].1)
            };
            ans = ans.min(tmp);
        }
    }
    ans
}
