#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::str::FromStr;
use std::{
    collections::HashSet,
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

fn solve(n: u64) -> u64 {
    let mut set = HashSet::new();
    let root = (n as f64).sqrt().ceil() as usize;
    for i in 2..=root {
        for j in 2..=root {
            let pow = (i as u64).pow(j as u32);
            if pow > n {
                break;
            }
            set.insert(pow);
        }
    }
    n - set.len() as u64
}
