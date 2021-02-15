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
    println!("{}", solve(n));
}

fn solve(n: usize) -> usize {
    let mut ans = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            continue;
        }
        let mut tmp = 0;
        for j in 1..=((i as f64).sqrt()).ceil() as usize {
            if i % j == 0 {
                tmp += 1;
                if j * j != i {
                    tmp += 1;
                }
            }
        }
        if tmp == 8 {
            ans += 1;
        }
    }
    ans
}
