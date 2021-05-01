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
    let T: usize = read();
    for t in 1..=T {
        let n: usize = read();
        println!(
            "Case #{}: {}",
            t,
            solve(read(), (0..n).map(|_| read()).collect())
        );
    }
}

fn solve(k: usize, mut p: Vec<usize>) -> f64 {
    p.sort();
    let mut counts = vec![0];
    let mut now = &p[0];
    for i in &p[1..] {
        if i - now <= 1 {
            now = i;
            continue;
        }
        counts.push(i - now - 1);
        now = i;
    }
    let front_point = p[0] - 1;
    let last_point = k - p[p.len() - 1];
    // 1個ずつ別々に取る場合
    let mut counts2 = vec![];
    for i in &counts {
        counts2.push((i + 1) / 2);
    }
    counts2.push(front_point);
    counts2.push(last_point);
    counts2.sort();
    counts2.reverse();
    let sum1 = counts2[0] + counts2[1];
    // 2個入れる場合
    let sum2 = counts.into_iter().max().unwrap();

    std::cmp::max(sum1, sum2) as f64 / k as f64
}
