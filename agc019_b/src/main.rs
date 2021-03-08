#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::str::FromStr;
use std::{
    collections::HashMap,
    io::{stdin, Read},
};
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
    println!("{}", solve(read()));
}

fn solve(s: String) -> u64 {
    let mut ans = 0;
    let mut map = HashMap::new();
    let mut count = 0;
    for c in s.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
        count += 1;
    }
    ans += count * (count - 1) / 2;
    // 何も変わらない場合
    ans += 1;
    for (_, count) in map {
        ans -= (count * (count - 1)) / 2
    }
    ans
}
