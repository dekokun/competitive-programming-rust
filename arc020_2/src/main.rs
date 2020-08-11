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
    let cost: usize = read();
    let vec: Vec<usize> = (0..n).map(|_| read()).collect();
    let mut change = n * cost;
    for i in 1..=10 {
        for j in 1..=10 {
            if i == j {
                continue;
            }
            for (i, j) in &[(i, j), (j, i)] {
                let mut tmp_change = 0;
                for (index, &v) in vec.iter().enumerate() {
                    if index % 2 == 0 && *i != v || index % 2 == 1 && *j != v {
                        tmp_change += 1;
                    }
                }
                change = std::cmp::min(change, tmp_change)
            }
        }
    }
    println!("{}", change * cost);
}
