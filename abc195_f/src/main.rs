#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::{HashMap, HashSet};

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

fn solve(a: u64, b: u64) -> u64 {
    let mut rem = HashSet::new();
    for i in a..=b {
        rem.insert(i);
    }

    let mut prime_factor_cache: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    for i in a..=b {
        let primes = prime_factorization(i);
        prime_factor_cache.insert(i, primes);
    }
    let prime_factor_cache = prime_factor_cache;
    let mut rem_cache = HashMap::new();
    dfs(rem, &prime_factor_cache, &mut rem_cache, &(a, b))
}

fn dfs(
    rem: HashSet<u64>,
    prime_factor_cache: &HashMap<u64, HashMap<u64, u64>>,
    rem_cache: &mut HashMap<Vec<u64>, u64>,
    &ab: &(u64, u64),
) -> u64 {
    if rem.is_empty() {
        return 1;
    }
    let mut key: Vec<u64> = rem.clone().into_iter().collect();
    key.sort();
    if rem_cache.contains_key(&key) {
        debug!("hoge");
        return rem_cache[&key];
    }
    let mut ans = 0;
    for &v in &rem {
        let mut rem2 = rem.clone();
        rem2.remove(&v);
        let primes = prime_factor_cache[&v].clone();
        let a = ab.0;
        let b = ab.1;
        for (prime, _) in primes {
            let first = a + (prime - (a % prime));
            // debug!(prime);
            for i in 0..50 {
                let now = first + i * prime;
                if now > b {
                    break;
                }
            }
        }
        let mut key: Vec<u64> = rem2.clone().into_iter().collect();
        key.sort();
        ans += dfs(rem2, prime_factor_cache, rem_cache, &ab);
    }
    rem_cache.insert(key, ans);
    debug!(rem_cache.len());
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
