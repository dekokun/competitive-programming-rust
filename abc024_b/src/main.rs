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
    let t: usize = read();
    println!("{}", solve(n, t, (0..n).map(|_| read()).collect()));
}

fn solve(n: usize, t: usize, a: Vec<usize>) -> usize {
    let mut imos = vec![false; a[n - 1] + t + 1];
    for &v in &a {
        imos[v] = true;
    }
    let mut ans = 0;
    let mut before = a[0];
    for (now, v) in imos.into_iter().enumerate() {
        if (before + t > now && now >= before) || v {
            ans += 1;
        }
        if v {
            before = now;
        }
    }
    ans
}
