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
    let h: usize = read();
    let w: usize = read();
    println!(
        "{}",
        solve(
            h,
            w,
            (0..h).map(|_| read::<String>().chars().collect()).collect()
        )
    );
}

fn solve(h: usize, w: usize, m: Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let mut tmp = 0;
            for &(a, b) in &[(0, 0), (1, 0), (0, 1), (1, 1)] {
                if m[i + a][j + b] == '#' {
                    tmp += 1;
                }
            }
            if tmp % 2 == 1 {
                ans += 1;
            }
        }
    }
    ans
}
