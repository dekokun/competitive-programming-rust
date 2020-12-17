#![allow(non_snake_case)]

use std::collections::HashMap;
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

fn prime_factorization(n: u64) -> HashMap<u64, u64> {
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

fn main() {
    let n: u64 = read();
    let p: u64 = read();

    let primes = prime_factorization(p);
    let mut ans = 1;
    for (key, v) in primes {
        if v >= n {
            ans *= key.pow((v / n) as u32);
        }
    }
    println!("{}", ans)
}
