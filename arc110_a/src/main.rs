#![allow(non_snake_case)]

use std::io::{stdin, Read};
use std::str::FromStr;
use std::collections::HashMap;
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

fn main() {
    let n: usize = read();

    let mut divisors: HashMap<usize, usize> = HashMap::new();
    for i in 1..=n {
        let primes = prime_factorization(i);
        for (key, v) in primes {
            match divisors.get(&key) {
                Some(&v2) => {
                    divisors.insert(key, v.max(v2));
                },
                None => {
                    divisors.insert(key, v);
                }
            }
        }
    }
    let mut ans = 1;
    for (key, v) in divisors {
        ans *= key.pow(v as u32) as u64;
    }
    println!("{}", ans + 1)
}
