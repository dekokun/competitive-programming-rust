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

fn mod_pow(x: u64, n: u64, MOD: u64) -> u64 {
    let mut ans = 1;
    let mut n = n;
    let mut x = x;
    while n != 0 {
        if n % 2 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let t: usize = read();
    'outer: for _ in 0..t {
        let n: u64 = read();
        let s: u64 = read();
        let k: u64 = read();

        if (n - s) % k == 0 {
            println!("{}", (n - s) / k);
            continue 'outer;
        }
        // n, k が互いに素だったら絶対に答えはあるし逆元もある(k < n)
        if gcd(n, k) == 1 {
            println!("{}", ((n - s) * mod_pow(k, n - 2, n)) % n);
            continue 'outer;
        }
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for i in 1..=n {
            let rem = (s + i * k) % n;
            if rem == 0 {
                println!("{}", i);
                continue 'outer;
            }
            if set.contains(&rem) {
                println!("-1");
                continue 'outer;
            }
            set.insert(rem);
        }
        println!("{}", -1);
    }
}
