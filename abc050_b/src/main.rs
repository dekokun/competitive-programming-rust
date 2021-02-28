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
    let t = (0..n).map(|_| read()).collect();
    let m: usize = read();

    println!(
        "{}",
        solve(t, (0..m).map(|_| (read(), read())).collect())
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(t: Vec<usize>, px: Vec<(usize, usize)>) -> Vec<usize> {
    let sum: usize = t.iter().sum();
    px.into_iter().map(|(p, x)| sum - t[p - 1] + x).collect()
}
