#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }
    println!("{}", solve(a, b, c, d, e, f, x));
}

fn solve(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, x: usize) -> String {
    let t_d = ((x / (a + c)) * a * b)
        + if (x % (a + c)) >= a {
            b * a
        } else {
            (x % (a + c)) * b
        };
    let a_d = ((x / (d + f)) * d * e)
        + if (x % (d + f)) >= d {
            e * d
        } else {
            (x % (d + f)) * e
        };
    if t_d == a_d {
        "Draw"
    } else if t_d > a_d {
        "Takahashi"
    } else {
        "Aoki"
    }
    .into()
}
