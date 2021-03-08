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

fn solve(n: u64, k: u64) -> u64 {
    let mut n = n;
    for _ in 0..k {
        let mut vec = vec![];
        let mut tmp = n;
        while tmp > 0 {
            vec.push(tmp % 10);
            tmp /= 10;
        }
        vec.sort();
        let mut g1 = 0;
        let mut multi = 1;
        for &v in &vec {
            g1 += v * multi;
            multi *= 10;
        }
        vec.reverse();
        let mut g2 = 0;
        let mut multi = 1;
        for &v in &vec {
            g2 += v * multi;
            multi *= 10;
        }
        n = g1 - g2;
    }
    n
}

/*
6174 7641 - 1467 = 6174

*/
