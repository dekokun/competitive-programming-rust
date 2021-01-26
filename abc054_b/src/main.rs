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
    let m: usize = read();

    println!(
        "{}",
        solve(
            (0..n).map(|_| read()).collect(),
            (0..m).map(|_| read()).collect()
        )
    );
}

fn solve(a: Vec<String>, b: Vec<String>) -> String {
    let n = a.len();
    let m = b.len();
    let a: Vec<Vec<char>> = a.into_iter().map(|a| a.chars().collect()).collect();
    let b: Vec<Vec<char>> = b.into_iter().map(|a| a.chars().collect()).collect();
    for i in 0..=(n - m) {
        'outer: for j in 0..=(n - m) {
            for k in 0..m {
                for l in 0..m {
                    if a[i + k][j + l] != b[k][l] {
                        continue 'outer;
                    }
                }
            }
            return "Yes".into();
        }
    }
    return "No".into();
}
