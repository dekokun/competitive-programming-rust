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
    let h: usize = read();
    let w: usize = read();
    let mut vec: Vec<Vec<usize>> = vec![];
    let mut min = 1000;
    for _ in 0..h {
        let mut row = vec![];
        for _ in 0..w {
            let val = read();
            row.push(val);
            min = min.min(val);
        }
        vec.push(row);
    }
    let mut ans = 0;
    for row in vec {
        for v in row {
            ans += v - min;
        }
    }
    println!("{}", ans)
}
