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
    let N: i32 = read();
    let K: usize = read();
    let mut vec: Vec<i32> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    let mut now_val = 0;
    let mut max = 0;
    let mut ans = 0.0;
    for (i, &v) in vec.iter().enumerate() {
        now_val += v;
        if i >= K {
            now_val -= vec[i - K];
        }
        if max < now_val {
            max = now_val;
            ans = K as f64 / 2.0 + max as f64 / 2.0;
        }
    }
    println!("{}", ans);
}
