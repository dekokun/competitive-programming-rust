#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::str::FromStr;
use std::{
    collections::HashSet,
    io::{stdin, Read},
};
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
    println!("{}", solve(read(), read(), read()));
}

fn solve(a: i64, b: i64, c: i64) -> u64 {
    // 周期をとってその周期で剰余をとる
    let a = a % 10;
    let mut set: HashSet<i64> = HashSet::new();
    set.insert(a);
    let mut tmp = a;
    for _ in 1..=11 {
        tmp *= a;
        tmp %= 10;
        set.insert(tmp);
    }
    debug!(set);
    let cycle = set.len();
    let d = pow_mod(b, c, cycle as i32);
    let d = if d == 0 { cycle as i64 } else { d };
    pow_mod(a, d, 10) as u64
}
#[allow(dead_code)]
fn solve2(a: i64, b: i64, c: i64) -> u64 {
    let pow = b.pow(c as u32);
    pow_mod(a, pow, 10) as u64
}

fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
    if m == 1 {
        return 0;
    }
    let _m = m as u32;
    let mut r: u64 = 1;
    let mut y: u64 = safe_mod(x, m as i64) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (_m as u64);
        }
        y = (y * y) % (_m as u64);
        n >>= 1;
    }
    r as i64
}
fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[test]
    fn test1() {
        for i in 1..10 {
            debug!(i);
            assert_eq!(solve(i, 2, 2), solve2(i, 2, 2));
        }
    }
}
