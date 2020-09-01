#![allow(non_snake_case)]

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
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let N: usize = read();
    let A: Vec<usize> = (0..N).map(|_| read()).collect();
    let MOD = 10_u64.pow(9) + 7;
    let mut lcm = HashMap::new();
    let mut primes = vec![];
    for v in &A {
        primes.push(prime_factorization(*v));
    }
    for primes in &primes {
        for (p, &count) in primes {
            let entry = lcm.entry(p).or_insert(0);
            *entry = max(*entry, count);
        }
    }
    let mut lcm_num = 1;
    for (p, count) in lcm {
        lcm_num *= bin_pow(*p as u64, count as u64, MOD);
        lcm_num %= MOD;
    }
    let mut ans = 0;
    for v in A {
        ans += lcm_num * inverse_mod(v as u64, MOD);
        ans %= MOD;
    }
    println!("{}", ans)
}

fn bin_pow(x: u64, n: u64, MOD: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        (bin_pow(x, n / 2, MOD) % MOD).pow(2) % MOD
    } else {
        bin_pow(x, n - 1, MOD) % MOD * x % MOD
    }
}

fn inverse_mod(n: u64, MOD: u64) -> u64 {
    bin_pow(n, MOD - 2, MOD)
}

fn prime_factorization(n: usize) -> HashMap<usize, usize> {
    let mut ans = HashMap::new();
    let mut n = n;
    for i in 2..=((n as f64).sqrt().ceil() as usize) {
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
