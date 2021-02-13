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
    println!("{}", solve(read(), read()));
}

fn solve(b: i64, c: i64) -> i64 {
    let left_left = -b - (c - 1) / 2;
    let left_right = -b + (c - 1) / 2;
    let right_left = b - c / 2;
    let right_right = b + ((c - 2) / 2).max(0);
    debug!(left_left, left_right, right_left, right_right);
    if left_right >= right_left {
        debug!("cover");
        right_right.max(left_right) - left_left.min(right_left) + 1
    } else {
        debug!("separate");
        left_right - left_left + 1 + right_right - right_left + 1
    }
}
