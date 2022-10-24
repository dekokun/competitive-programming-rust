#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;
use proconio::marker::Chars;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }
    println!("{}", solve(h, w, c).into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
}

fn solve(_: usize, w: usize, c: Vec<Vec<char>>) -> Vec<usize> {
    let mut ans = vec![0; w];
    for v in c.iter() {
        for (i, c) in v.into_iter().enumerate() {
            if c == &'#' {
                ans[i] += 1;
            }
        }
    }
    ans
}
