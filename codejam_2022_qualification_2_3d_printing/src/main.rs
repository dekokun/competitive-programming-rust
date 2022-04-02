#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/
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
    for i in 1..=n {
        let mut tmp = vec![];
        for _ in 0..3 {
            tmp.push((read(), read(), read(), read()));
        }
        println!(
            "Case #{}: {}",
            i,
            solve(tmp)
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn solve(a: Vec<(usize, usize, usize, usize)>) -> Vec<String> {
    let mut cis = vec![];
    let mut mis = vec![];
    let mut yis = vec![];
    let mut kis = vec![];
    for v in a {
        cis.push(v.0);
        mis.push(v.1);
        yis.push(v.2);
        kis.push(v.3);
    }
    let cmin = cis.into_iter().min().unwrap();
    let mmin = mis.into_iter().min().unwrap();
    let ymin = yis.into_iter().min().unwrap();
    let kmin = kis.into_iter().min().unwrap();
    let max = 10_usize.pow(6);
    if cmin + mmin + ymin + kmin >= max {
        let c = cmin;
        let m = if c + mmin >= max { max - c } else { mmin };
        let y = if c + m + ymin >= max {
            max - c - m
        } else {
            ymin
        };
        let k = max - c - m - y;
        vec![c, m, y, k]
            .into_iter()
            .map(|a| a.to_string())
            .collect()
    } else {
        vec!["IMPOSSIBLE".to_string()]
    }
}
