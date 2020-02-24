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
    let n: usize = read();
    let a: usize = read();
    let b: usize = read();
    let M: usize = 10_usize.pow(9) + 7;
    let max = pow(2, n, M);
    dbg!(max, combination(n, a, M), combination(n, b, M));
    let mut ans =
        (max as i32 - 1 - combination(n, a, M) as i32 - combination(n, b, M) as i32) % M as i32;
    if ans < 0 {
        ans += M as i32;
    }
    println!("{}", ans);
}

#[test]
fn combi_works1() {
    assert_eq!(combination(6, 2, 17), 15);
}
#[test]
fn combi_works2() {
    assert_eq!(combination(6, 2, 13), 2);
}
#[test]
fn combi_works3() {
    assert_eq!(combination(6, 2, 11), 4);
}
fn combination(n: usize, k: usize, m: usize) -> usize {
    if n == 0 && k == 0 {
        return 1;
    }
    if n < k {
        return 0;
    }
    let k_fact = factorial(k, m);
    let mut ans = 1;
    for i in n - k + 1..n + 1 {
        ans *= i;
        ans %= m;
    }
    (ans * pow(k_fact, m - 2, m)) % m
}

#[test]
fn pow_works1() {
    assert_eq!(pow(5, 2, 100), 25);
}
#[test]
fn pow_works2() {
    assert_eq!(pow(5, 2, 20), 5);
}
fn pow(a: usize, b: usize, m: usize) -> usize {
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

#[test]
fn factorial_works1() {
    assert_eq!(factorial(4, 100), 24);
}
#[test]
fn factorial_works2() {
    assert_eq!(factorial(4, 10), 4);
}
fn factorial(a: usize, m: usize) -> usize {
    let mut ans = 1;
    for i in 1..a + 1 {
        ans *= i;
        ans %= m;
    }
    ans
}
