#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::str::FromStr;
use std::{
    collections::BTreeSet,
    io::{stdin, Read},
};
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
    let m = read();
    println!("{}", solve(m, (0..n).map(|_| read()).collect()));
}

fn solve(m: usize, a: Vec<usize>) -> usize {
    let mut set = BTreeSet::new();
    for i in 0..=a.len() {
        set.insert(i);
    }
    let mut count = 0;
    let mut ans = a.len();
    let mut counts = vec![0; a.len() + 1];
    for (i, &v) in a.iter().enumerate() {
        set.remove(&v);
        counts[v] += 1;
        if count < m {
            count += 1;
            if count == m {
                ans = ans.min(*set.iter().next().unwrap());
                debug!(set, v, ans, "=m");
            }
            continue;
        }
        // m個溜まってる状態
        counts[a[i - m]] -= 1;
        if counts[a[i - m]] == 0 {
            set.insert(a[i - m]);
        }
        ans = ans.min(*set.iter().next().unwrap());
        debug!(set, v, ans, ">m");
    }
    ans
}
