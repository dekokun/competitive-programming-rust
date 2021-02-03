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
    let n: usize = read();
    let a = read();
    let b = read();
    let k: usize = read();
    println!("{}", solve(n, a, b, (0..k).map(|_| read()).collect()));
}

fn solve(n: usize, a: usize, b: usize, p: Vec<usize>) -> String {
    let mut visited = vec![false; n + 1];
    visited[a] = true;
    visited[b] = true;
    for v in p {
        if visited[v] {
            return "NO".into();
        }
        visited[v] = true;
    }
    return "YES".into();
}
