#![allow(non_snake_case, unused_macros, dead_code)]

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
    let t: usize = read();
    for i in 1..=t {
        let n: usize = read();

        println!("Case #{}: {}", i, solve((0..n).map(|_| read()).collect()));
    }
}

fn solve(mut l: Vec<usize>) -> usize {
    let mut ans = 0;
    for i in 0..l.len() - 1 {
        let mut min_pos = l.len() - 1;
        let mut min = l[min_pos];
        for j in i..l.len() {
            if l[j] < min {
                min = l[j];
                min_pos = j;
            }
        }
        let sl = &l[i..=min_pos];
        let mut sl = sl.to_vec();
        sl.reverse();
        for (i2, v) in sl.into_iter().enumerate() {
            l[i + i2] = v;
            ans += 1;
        }
    }
    ans
}
