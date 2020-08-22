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
    if N == 1 {
        println!("Not Prime");
        return;
    }
    for i in 2..(((N as f64).sqrt() + 1.0) as usize) {
        if N % i == 0 {
            let digits: Vec<usize> = N
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            let last_digit = digits[digits.len() - 1];
            if last_digit % 2 == 0 || last_digit == 5 {
                println!("Not Prime");
                return;
            }
            if digits.into_iter().sum::<usize>() % 3 == 0 {
                println!("Not Prime");
                return;
            }
            break;
        }
    }
    println!("Prime");
}