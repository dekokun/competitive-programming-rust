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
    let X: usize = read();
    let Y: usize = read();
    let A: usize = read();
    let B: usize = read();
    let C: usize = read();
    let mut As: Vec<usize> = (0..A).map(|_| read()).collect();
    As.sort();
    As.reverse();
    let mut Bs: Vec<usize> = (0..B).map(|_| read()).collect();
    Bs.sort();
    Bs.reverse();
    let Cs: Vec<usize> = (0..C).map(|_| read()).collect();
    let mut all: Vec<usize> = Cs;
    all.extend_from_slice(&As[0..X]);
    all.extend_from_slice(&Bs[0..Y]);
    all.sort();
    all.reverse();
    let ans: usize = all[0..X + Y].iter().sum();
    println!("{}", ans)
}
