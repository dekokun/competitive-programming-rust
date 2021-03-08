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
    let n = read();
    println!(
        "{}",
        solve(
            n,
            read(),
            (0..n).map(|_| (read(), read(), read())).collect()
        )
    );
}

fn solve(_n: u64, c: i64, vec: Vec<(usize, usize, i64)>) -> u64 {
    use std::collections::BTreeMap;
    let mut imos: BTreeMap<usize, i64> = BTreeMap::new();
    let max = *vec.iter().map(|(_a, b, _c)| b).max().unwrap();
    for (a, b, c) in vec {
        let entry = imos.entry(a).or_insert(0);
        *entry += c;
        let entry = imos.entry(b + 1).or_insert(0);
        *entry -= c;
    }
    let mut ans: u64 = 0;
    let mut now = 0;
    let mut before = 1;
    for (k, v) in imos {
        ans += (if now >= c { c as u64 } else { now as u64 }) * (k - before) as u64;
        now += v;
        before = k;
    }
    ans += (if now >= c { c as u64 } else { now as u64 }) * (max + 1 - before) as u64;
    ans
}
