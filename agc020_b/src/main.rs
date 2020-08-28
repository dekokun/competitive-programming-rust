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
    let K: usize = read();
    let A: Vec<usize> = (0..K).map(|_| read()).rev().collect();
    if A[0] > 2 {
        println!("{}", -1);
        return;
    }
    let mut before = 2;
    for &a in &A {
        if a >= before * 2 {
            println!("{}", -1);
            return;
        }
        before = a;
    }
    let mut min = *A.iter().max().unwrap();
    let mut max = 0;
    let mut a_max = A[0];
    for &a in &A[1..] {
        if a > a_max {
            max = a * 2 - 1;
            a_max = a;
            min = a;
        } else if a_max % a == 0 {
            continue;
        } else {
            // maxは、a_maxの倍にならない範囲で増える
            // minも増える
        }
    }
    println!("{} {}", min, max)
}
