#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        s: String
    }
    println!("{}", solve(n, s));
}

fn solve(n: usize, s: String) -> usize {
    let next_x = next('x', &s);
    let next_o = next('o', &s);
    let mut ans = 0;
    for (i, c) in s.chars().enumerate() {
        let next = if c == 'x' { &next_o } else { &next_x };
        let next = next[i];
        ans += if next.is_none() {
            0
        } else {
            let next = next.unwrap();
            n - next
        };
    }
    ans
}

fn next(cc: char, s: &String) -> Vec<Option<usize>> {
    let n = s.len();
    s.chars()
        .rev()
        .enumerate()
        .fold(vec![], |mut acc, (i, c)| {
            if c == cc {
                acc.push(Some(n - i - 1));
                acc
            } else if acc.is_empty() {
                vec![None]
            } else {
                acc.push(acc[acc.len() - 1]);
                acc
            }
        })
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
}
