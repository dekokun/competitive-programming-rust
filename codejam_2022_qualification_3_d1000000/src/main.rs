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
        let c = read();
        println!("Case #{}: {}", i, solve((0..c).map(|_| read()).collect()));
    }
}

fn solve(mut a: Vec<usize>) -> usize {
    a.sort();
    let mut ans = 0;
    let mut now = 0;
    for i in a {
        if i > now {
            now += 1;
            ans += 1;
        }
    }
    ans
}
