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
    let n = read();
    println!(
        "{}",
        solve(n, (0..2_usize.pow(n as u32)).map(|_| read()).collect())
    );
}

fn solve(n: usize, a: Vec<usize>) -> usize {
    let mut a: Vec<(usize, usize)> = a.into_iter().enumerate().collect();
    a.sort_by_key(|&a| a.1);
    a.reverse();
    let one = a[0];

    // one.0 => index, one.1 => rate
    let split = 2_usize.pow((n - 1) as u32);
    for &(index, _rate) in a.iter().skip(1) {
        if (one.0 < split && index >= split) || (one.0 >= split && index < split) {
            return index + 1;
        }
    }
    panic!();
}
