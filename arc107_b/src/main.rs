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

use std::cmp::{max, min};
fn main() {
    let n: i64 = read();
    let k: i64 = read();
    let sum_min = max(2, k + 2);
    let sum_max = min(2 * n, 2 * n + k);
    let mut ans = 0;
    for ab in sum_min..=sum_max {
        let cd = ab - k;
        let tmp = (min(ab - 1, n) - max(ab - n, 1) + 1) * (min(cd - 1, n) - max(cd - n, 1) + 1);
        // dbg!(ab, cd, tmp);
        ans += tmp;
    }
    println!("{}", ans);
}
