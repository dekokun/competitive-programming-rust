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
    let n: u64 = read();
    let m: u64 = read();
    let s: Vec<char> = read::<String>().chars().collect();
    let t: Vec<char> = read::<String>().chars().collect();
    let lcm = lcm(n, m);
    use std::collections::HashMap;
    let mut x: HashMap<u64, char> = HashMap::new();
    for (i, c) in s.into_iter().enumerate() {
            x.insert((lcm / n) * i as u64, c);
    }
    for (i, c) in t.into_iter().enumerate() {
        if let Some(&c2) = x.get(&((lcm / m) * i as u64)) {
            if c2 != c {
                println!("-1");
                return
            }
        }
    }
    println!("{}", lcm);
}

fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);
    let a = a;
    let b = b;
    (a * b) / gcd
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}
