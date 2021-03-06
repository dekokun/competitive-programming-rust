#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ $B$+$i(B
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
    println!("{}", solve((0..n).map(|_| read()).collect()));
}

fn solve(a: Vec<i64>) -> i64 {
    // 全部の和と全部の二乗和を求める
    // a(b + c + d) + b(c + d) + c(d) のように、和から一つずつ引きつつかけていく
    let sum_sq: i64 = a.iter().map(|v| v.pow(2)).sum();
    let mut sum: i64 = a.iter().sum();
    let mut tmp = 0;
    for &v in &a {
        sum -= v;
        tmp += -2 * sum * v;
    }
    debug!(sum_sq, tmp);
    (a.len() - 1) as i64 * sum_sq + tmp
}

fn solve2(a: Vec<i64>) -> i64 {
    let mut ans = 0;
    for i in 1..a.len() {
        for j in 0..i {
            debug!(a[i].pow(2), a[j].pow(2), -2 * a[i] * a[j]);
            ans += (a[i]).pow(2) + a[j].pow(2) - 2 * a[i] * a[j];
        }
    }
    ans
}
