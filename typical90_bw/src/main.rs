#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

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

fn solve(n: u64) -> usize {
    let primes = prime_factorization(n);
    let prime_count = primes.into_iter().fold(0, |acc, (_i, v)| acc + v);
    let mut now = prime_count;
    let mut ans = 0;
    while now != 1 {
        ans += 1;
        now = (now + 1) / 2;
    }
    ans
}

fn prime_factorization(n: u64) -> HashMap<u64, usize> {
    let mut ans = HashMap::new();
    let mut n = n;
    for i in 2..=((n as f64).sqrt().ceil() as u64) {
        while n % i == 0 && n != 1 {
            n /= i;
            let entry = ans.entry(i).or_insert(0);
            *entry += 1;
        }
    }
    if n != 1 {
        let entry = ans.entry(n).or_insert(0);
        *entry += 1;
    }
    ans
}
