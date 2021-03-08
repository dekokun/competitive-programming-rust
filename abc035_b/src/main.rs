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
    println!("{}", solve(read::<String>().chars().collect(), read()));
}

fn solve(s: Vec<char>, t: usize) -> i32 {
    let mut pos = (0, 0);
    let mut skipped = 0;
    for c in s {
        let (x, y) = pos;
        pos = match c {
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            '?' => {
                skipped += 1;
                (x, y)
            }
            _ => panic!(),
        };
    }
    let (x, y): (i32, i32) = pos;
    let dist = x.abs() + y.abs();
    if t == 1 {
        dist + skipped
    } else if dist >= skipped {
        dist - skipped
    } else {
        (skipped - dist) % 2
    }
}
