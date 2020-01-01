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
    let N: u32 = read();
    let mut vec: Vec<u64> = vec![];
    for i in 0..N {
        let v = read();
        if (i == 0 && v != 0) || (i != 0 && v == 0) {
            println!("0");
            return;
        }
        vec.push(v);
    }
    vec.sort();
    // (v, count)
    let mut v2: Vec<(u64, u64)> = vec![];
    for v in vec {
        if v2.is_empty() {
            v2.push((v, 1));
        } else if v2[v2.len() - 1].0 != v {
            if v - v2[v2.len() - 1].0 != 1 {
                println!("0");
                return;
            }
            v2.push((v, 1));
        } else {
            let (last_v, last_count) = v2.pop().unwrap();
            v2.push((last_v, last_count + 1));
        }
    }
    let mut before = 1u64;
    let mut ans = 1;
    for (_, count) in v2 {
        ans *= mod_pow(before, count);
        ans %= 998_244_353;
        before = count;
    }
    println!("{}", ans);
}

fn mod_pow(base: u64, exp: u64) -> u64 {
    let mut ans = 1;
    for _ in 0..exp {
        ans *= base;
        ans %= 998_244_353;
    }
    ans
}
