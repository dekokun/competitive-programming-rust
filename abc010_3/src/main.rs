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
    let tx1: i32 = read();
    let ty1: i32 = read();
    let tx2: i32 = read();
    let ty2: i32 = read();
    let T: i32 = read();
    let V: i32 = read();
    let n: i32 = read();
    for _ in 0..n {
        let x: i32 = read();
        let y: i32 = read();
        if ((tx1 - x).pow(2) as f64 + (ty1 - y).pow(2) as f64).sqrt()
            + ((tx2 - x).pow(2) as f64 + (ty2 - y).pow(2) as f64).sqrt()
            <= (V * T) as f64
        {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
