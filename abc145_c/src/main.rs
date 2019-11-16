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
    let N: usize = read();
    let mut vec = vec![];
    for _ in 0..N {
        let x: f64 = read();
        let y: f64 = read();
        vec.push((x, y));
    }
    let mut sum = 0.0;
    for &(x1, y1) in &vec {
        for &(x2, y2) in &vec {
            sum += ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
        }
    }
    let ans = sum * factorial(N as i64 - 1) as f64 / factorial(N as i64) as f64;
    println!("{}", ans);
}

fn factorial(n: i64) -> i64 {
    let mut ans = 1;
    for i in 1..n + 1 {
        ans *= i;
    }
    ans
}
