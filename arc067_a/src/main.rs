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

fn main() {
    const MOD: usize = 1_000_000_000 + 7;
    let N: usize = read();
    let mut primes = vec![];
    use std::collections::HashMap;
    let mut factors = HashMap::new();
    for i in 2..N + 1 {
        let mut is_composite = false;
        for p in &primes {
            if i % p == 0 {
                is_composite = true;
                let mut i = i;
                let mut add = 0;
                while i % p == 0 {
                    add += 1;
                    i /= *p;
                }
                let entry = factors.entry(p.clone()).or_insert(0);
                *entry += add;
            }
        }
        if is_composite {
            continue;
        }
        factors.insert(i, 1);
        primes.push(i);
    }

    let ans = factors
        .iter()
        .fold(1, |acc, (&_key, &value)| acc * (value + 1) % MOD);
    println!("{}", ans);
}
