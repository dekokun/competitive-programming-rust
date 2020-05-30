#![allow(non_snake_case)]

use std::cmp::min;
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
    let vec: Vec<_> = (0..=N).map(|_| read::<u64>()).collect();
    if N == 0 && vec[0] == 1 {
        println!("1");
        return;
    }
    if vec[0] != 0 {
        println!("-1");
        return;
    }
    let mut before = 1;
    for &v in &vec {
        if v > before {
            println!("-1");
            return;
        }
        before -= v;
        before *= 2;
    }
    let mut sum: u64 = vec.iter().sum();
    let mut ans: u64 = 1;
    let mut now_ver: u64 = 1;
    for &v in vec.iter().skip(1) {
        now_ver = min(now_ver.saturating_mul(2), sum);
        ans += now_ver;
        now_ver -= v;
        sum -= v;
    }
    println!("{}", ans)
}
