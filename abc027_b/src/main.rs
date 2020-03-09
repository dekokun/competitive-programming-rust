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
    let mut vec: Vec<i32> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    let sum = vec.iter().sum::<i32>();
    if sum % N != 0 {
        println!("-1");
        return;
    }
    let avg = sum / N;
    let mut ans = 0;
    let mut now_val = 0;
    for v in vec {
        now_val += avg - v;
        if now_val != 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
