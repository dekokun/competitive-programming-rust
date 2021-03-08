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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}

fn main() {
    let s = read();
    let n: usize = read();
    println!(
        "{}",
        solve(s, n, (0..n).map(|_| (read(), read())).collect())
    );
}

fn solve(s: String, _n: usize, lr: Vec<(usize, usize)>) -> String {
    let mut s: Vec<_> = s.chars().collect();
    for (l, r) in lr {
        let s2 = s.clone();
        for (i, &c) in (&s2[l - 1..r]).iter().rev().enumerate() {
            debug!(i, l - 1 + i);
            s[l - 1 + i] = c;
        }
        debug!(s);
    }
    s.into_iter().collect()
}
