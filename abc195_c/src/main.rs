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
        n: u64,
    }
    println!("{}", solve(n));
}

fn solve(n: u64) -> u64 {
    let mut ans = 0;
    for i in 1..=6 {
        if n / (10_u64.pow(i * 3)) != 0 {
            ans += (i as u64 - 1) * (10_u64.pow(i * 3) - 10_u64.pow((i - 1) * 3));
        } else {
            ans += (n - 10_u64.pow((i - 1) * 3) + 1) * (i as u64 - 1);
            break;
        }
    }
    ans
}
