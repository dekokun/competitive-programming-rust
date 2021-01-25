#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::io::{stdin, Read};
use std::{collections::BTreeMap, str::FromStr};
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

    println!(
        "{}",
        solve(n, read(), (0..n).map(|_| (read(), read())).collect())
    );
}

fn solve(_n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for (a, b) in ab {
        let entry = map.entry(a).or_insert(0);
        *entry += b;
    }
    let mut count = 0;
    for (i, v) in map {
        count += v;
        if count >= k {
            return i;
        }
    }
    panic!();
}
