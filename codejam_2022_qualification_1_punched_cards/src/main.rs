#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/
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
    for i in 1..=n {
        let (r, c) = (read(), read());
        println!("Case #{}:\n{}", i, solve(r, c).join("\n"));
    }
}

fn solve(r: usize, c: usize) -> Vec<String> {
    let mut ans = vec![];
    let mut tmp = "..+".to_owned();
    for _ in 0..c - 1 {
        tmp += "-+";
    }
    ans.push(tmp);
    for i in 0..r {
        let mut tmp = "".to_owned();
        if i == 0 {
            tmp += "..|";
            for _ in 0..c - 1 {
                tmp += ".|";
            }
        } else {
            tmp += "|";
            for _ in 0..c {
                tmp += ".|";
            }
        }
        ans.push(tmp);
        tmp = "+".to_owned();
        for _ in 0..c {
            tmp += "-+";
        }
        ans.push(tmp);
    }
    ans
}
