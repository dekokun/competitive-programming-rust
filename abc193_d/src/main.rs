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

fn main() {
    println!("{}", solve(read(), read(), read()));
}

fn solve(k: usize, s: String, t: String) -> f64 {
    let mut t_set = vec![0; 10];
    let mut a_set = vec![0; 10];
    let mut remain = vec![k; 10];
    for c in s.chars() {
        if c == '#' {
            continue;
        }
        t_set[c.to_digit(10).unwrap() as usize] += 1;
        remain[c.to_digit(10).unwrap() as usize] -= 1;
    }
    for c in t.chars() {
        if c == '#' {
            continue;
        }
        a_set[c.to_digit(10).unwrap() as usize] += 1;
        remain[c.to_digit(10).unwrap() as usize] -= 1;
    }
    let remain = remain;
    let mut takahashi = 0;
    let mut aoki = 0;
    for i in 1..=9 {
        takahashi += i * 10_usize.pow(t_set[i]);
    }
    for i in 1..=9 {
        aoki += i * 10_usize.pow(a_set[i]);
    }
    let mut t_win = 0;
    let mut all = 0;
    for i in 1..=9 {
        let mut t = takahashi;
        t -= i * 10_usize.pow(t_set[i]);
        t += i * 10_usize.pow(t_set[i] + 1);
        for j in 1..=9 {
            if (i == j && remain[i] < 2) || remain[i] < 1 || remain[j] < 1 {
                continue;
            }
            let mut a = aoki;
            a -= j * 10_usize.pow(a_set[j]);
            a += j * 10_usize.pow(a_set[j] + 1);
            let kumiawase = if i == j {
                remain[i] * (remain[i] - 1)
            } else {
                remain[i] * remain[j]
            };
            if t > a {
                debug!(i, j, kumiawase);
                t_win += kumiawase;
            }
            debug!(i, j, kumiawase);
            all += kumiawase;
        }
    }
    debug!(all);
    t_win as f64 / (all as f64)
}
