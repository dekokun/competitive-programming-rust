#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

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
    }
    println!("{}", solve(a, b));
}

fn solve(a: u64, b: u64) -> u64 {
    let mut q = VecDeque::new();
    let mut rem = HashSet::new();
    for i in a..=b {
        rem.insert(i);
    }
    q.push_front(rem);
    // {} の文を最初に数えておく
    let mut ans = 1;
    let mut p_f: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    while let Some(rem) = q.pop_back() {
        if rem.is_empty() {
            ans += 1;
            continue;
        }
        for &v in &rem {
            let mut rem2 = rem.clone();
            rem2.remove(&v);
            let primes = if p_f.contains_key(&v) {
                p_f[&v].clone()
            } else {
                let primes = prime_factorization(v);
                p_f.insert(v, primes.clone());
                primes
            };

            for (prime, _) in primes {
                let first = a + (prime - (a % prime));
                debug!(prime);
                for i in 0..50 {
                    let now = first + i * prime;
                    if now > b {
                        break;
                    }
                    debug!(rem2.remove(&now));
                }
            }
            q.push_front(rem2);
        }
    }
    ans
}

fn prime_factorization(n: u64) -> HashMap<u64, u64> {
    debug!(n);
    let mut ans = HashMap::new();
    let mut n = n;
    for i in 2..=((n as f64).sqrt().ceil() as usize) {
        let i = i as u64;
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
