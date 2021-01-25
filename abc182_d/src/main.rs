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
    println!("{}", solve(n, (0..n).map(|_| read()).collect()));
}

fn solve(_n: usize, a: Vec<i64>) -> i64 {
    let mut cumsum = vec![];
    let mut before = 0;
    let mut local_max = 0;
    for v in a {
        let now = before + v;
        local_max = local_max.max(now);
        cumsum.push((now, local_max));
        before = now;
    }
    let mut max = 0;
    let mut now = 0;
    for v in cumsum {
        max = max.max(now + v.1);
        now += v.0;
    }
    max
}
