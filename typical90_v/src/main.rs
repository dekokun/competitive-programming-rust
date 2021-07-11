#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    println!("{}", solve(a, b, c));
}

fn solve(a: u64, b: u64, c: u64) -> u64 {
    let size = gcd(gcd(a, b), c);
    (a / size) - 1 + (b / size) - 1 + (c / size) - 1
}
fn gcd(a: u64, b: u64) -> u64 {
    // a の方が大きい
    let (a, b) = if a < b { (b, a) } else { (a, b) };
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
