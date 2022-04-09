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
    let T: usize = read();
    for t in 1..=T {
        let s = read();
        println!("Case #{}: {}", t, solve(s));
    }
}

fn solve(s: String) -> String {
    let mut vec = vec![];
    let mut now = None;
    for c in s.chars() {
        now = match now {
            None => Some((c, 1)),
            Some((a, b)) => {
                if a == c {
                    Some((c, b + 1))
                } else {
                    vec.push(now);
                    Some((c, 1))
                }
            }
        }
    }
    vec.push(now);
    let mut ans = "".to_string();
    for cs in vec.windows(2) {
        let before = cs[0].unwrap();
        let after = cs[1].unwrap();
        if before.0 < after.0 {
            for _ in 0..before.1 * 2 {
                ans.push(before.0);
            }
        } else {
            for _ in 0..before.1 {
                ans.push(before.0);
            }
        }
    }
    let last = vec[vec.len() - 1].unwrap();
    for _ in 0..last.1 {
        ans.push(last.0);
    }
    ans
}
