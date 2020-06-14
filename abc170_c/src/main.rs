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
    let x: i32 = read();
    let n: usize = read();
    let vec: Vec<i32> = (0..n).map(|_| read()).collect();
    for i in 0..102 {
        if !vec.contains(&(x - i)) {
            println!("{}", x - i);
            return;
        }
        if !vec.contains(&(x + i)) {
            println!("{}", x + i);
            return;
        }
    }
}
