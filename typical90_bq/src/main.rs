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
        n: i64,
        k: i64,
    }
    println!("{}", solve(n, k));
}

fn solve(n: i64, k: i64) -> i64 {
    let MOD = 10_i64.pow(9) + 7;
    if n == 1 {
        return k;
    }
    if n == 2 {
        return k * (k - 1);
    }
    ((k * (k - 1)) % MOD * mod_pow(k - 2, n - 2, MOD)) % MOD
}

fn mod_pow(x: i64, n: i64, MOD: i64) -> i64 {
    let mut ans = 1;
    let mut n = n;
    let mut x = x;
    while n != 0 {
        if n % 2 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}
