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
        a: u64,
        b: u64,
    }
    println!("{}", solve(a, b));
}

fn solve(a: u64, b: u64) -> String {
    let gcd = gcd(a, b);
    match (a / gcd).checked_mul(b) {
        None => "Large".into(),
        Some(a) if a > 10_u64.pow(18) => "Large".into(),
        Some(a) => a.to_string(),
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let (a, b) = if a < b { (b, a) } else { (a, b) };
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
