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
    let mut vec: Vec<(i32, i32)> = vec![];
    for _ in 0..N {
        vec.push((read(), read()));
    }

    use std::cmp::{max, min};
    let mut z_max = std::i32::MIN;
    let mut z_min = std::i32::MAX;
    let mut w_max = std::i32::MIN;
    let mut w_min = std::i32::MAX;
    for (x, y) in vec {
        let z = x - y;
        let w = x + y;
        z_max = max(z_max, z);
        z_min = min(z_min, z);
        w_max = max(w_max, w);
        w_min = min(w_min, w);
    }
    println!("{}", max(z_max - z_min, w_max - w_min));
}
