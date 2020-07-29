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
    let N: usize = read();
    let mut M: usize = read();
    if M % N == 0 {
        println!("{}", M / N);
        return;
    }
    use std::collections::HashMap;
    let mut primeFactor = HashMap::new();
    for i in 2..((M as f64).sqrt().ceil() as usize) {
        while M % i == 0 {
            let entry = primeFactor.entry(i).or_insert(0);
            *entry += 1;
            M /= i;
        }
    }
    if M != 1 {
        let entry = primeFactor.entry(M).or_insert(0);
        *entry += 1;
    }
    let mut ans = 1;
    let mut keys: Vec<_> = primeFactor.keys().collect();
    keys.sort();
    let mut split_key = None;
    for key in keys {
        let mut val = key.pow(*primeFactor.get(&key).unwrap());
        if split_key.is_none() && val >= N {
            split_key = Some(key);
            val /= key * std::cmp::max(N / key, 1);
        }
        ans *= val;
    }
    if split_key.is_none() {
        ans = 1;
    }
    println!("{}", ans)
}
