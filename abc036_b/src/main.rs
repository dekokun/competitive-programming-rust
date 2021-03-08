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

    println!(
        "{}",
        solve((0..n).map(|_| read::<String>().chars().collect()).collect())
            .into_iter()
            .map(|s| s.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(m: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = m.len();
    let mut ans = vec![];
    for i in 0..n {
        let mut row = vec![];
        for j in 0..n {
            row.push(m[n - j - 1][i]);
        }
        ans.push(row);
    }
    ans
}
