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
    let mut primes = HashMap::new();
    for i in 1..=n {
        let map = prime_factorization(i);
        for (k, v) in map {
            let entry = primes.entry(k).or_insert(0);
            *entry += v;
        }
    }
    // primesから、5, 5, 3(= 4,4,2)を満たすものを探す。
    println!("{}", num(75, &primes) + num(25, &primes) * (num(3, &primes) - 1) + num(15, &primes) * (num(5, &primes) - 1) + num(5, &primes) * (num(5, &primes) - 1)* (num(3, &primes) - 2) / 2);
}

fn num(m: usize, primes: &HashMap<usize, usize>) -> usize {
    primes.into_iter().filter(|(_k, &v)| v >= m - 1).count()
}
