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

fn mod_pow(a: i64, b: i64, m: i64) -> i64 {
    let mut ans = 1;
    let mut b = b;
    let mut a = a;
    while b != 0 {
        if b % 2 == 1 {
            ans = (ans * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    ans
}

fn main() {
    let N: i64 = read();
    let MOD: i64 = 10_i64.pow(9) + 7;
    let mut ans = (mod_pow(10, N, MOD) - mod_pow(9, N, MOD) * 2 + mod_pow(8, N, MOD)) % MOD;
    if ans < 0 {
        ans += MOD;
    }
    println!("{}", ans);
}
