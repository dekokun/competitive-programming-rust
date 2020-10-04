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
    let n: usize = read();
    let mut vec: Vec<i32> = vec![0; 2 * n];
    for i in 1..=n {
        let a: i32 = read();
        let b: i32 = read();
        if a > 0 && b > 0 && a >= b {
            println!("No");
            return;
        }
        if (a > 0 && vec[a as usize] != 0) || (b > 0 && vec[b as usize] != 0) {
            println!("No");
            return;
        }
        if a > 0 {
            vec[a as usize] = i as i32;
        }
        if b > 0 {
            vec[b as usize] = -(i as i32);
        }
    }
}
