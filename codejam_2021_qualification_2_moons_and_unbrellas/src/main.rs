#![allow(non_snake_case, unused_macros, dead_code)]

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
    let t: usize = read();
    for i in 1..=t {
        println!("Case #{}: {}", i, solve(read(), read(), read()));
    }
}

fn solve(x: usize, y: usize, s: String) -> usize {
    if s.chars().filter(|&c| c == '?').count() >= s.len() - 1 {
        return 0;
    }
    let mut s = s.chars().collect::<Vec<_>>();
    let mut before = '?';
    for i in 0..s.len() {
        let c = s[i];
        if c == '?' && before != '?' {
            s[i] = before;
            continue;
        }
        before = c;
    }
    s.reverse();
    before = '?';
    for i in 0..s.len() {
        let c = s[i];
        if c == '?' && before != '?' {
            s[i] = before;
            continue;
        }
        before = c;
    }
    s.reverse();
    let mut ans = 0;
    for cs in s.windows(2) {
        let c1 = cs[0];
        let c2 = cs[1];
        ans += match (c1, c2) {
            ('C', 'J') => x,
            ('J', 'C') => y,
            (_, _) => 0,
        };
    }
    ans
}
