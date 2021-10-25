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
        h: usize,
        w: usize,
        map: [[usize; w]; h]
    }
    println!("{}", solve(h, w, map));
}

fn solve(h: usize, w: usize, m: Vec<Vec<usize>>) -> String {
    for i1 in 0..h {
        for i2 in i1 + 1..h {
            for j1 in 0..w {
                for j2 in j1 + 1..w {
                    if m[i1][j1] + m[i2][j2] > m[i2][j1] + m[i1][j2] {
                        return "No".into();
                    }
                }
            }
        }
    }
    "Yes".into()
}
