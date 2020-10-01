#![allow(non_snake_case)]

use std::cmp::{max, min};
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
    let w: usize = read();
    let h: usize = read();
    let n: usize = read();
    let mut w_max = w;
    let mut h_max = h;
    let mut w_min = 0;
    let mut h_min = 0;
    for _ in 0..n {
        let (x, y, a): (usize, usize, usize) = (read(), read(), read());
        match a {
            1 => w_min = max(x, w_min),
            2 => w_max = min(x, w_max),
            3 => h_min = max(y, h_min),
            4 => h_max = min(y, h_max),
            _ => unreachable!(),
        }
    }
    println!(
        "{}",
        if w_max < w_min || h_max < h_min {
            0
        } else {
            (w_max - w_min) * (h_max - h_min)
        }
    );
}
