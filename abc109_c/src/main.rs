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

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let N: usize = read();
    let X: usize = read();
    let mut vec: Vec<usize> = vec![];
    vec.push(X);
    for _ in 0..N {
        vec.push(read());
    }
    vec.sort();
    let mut ans = vec[1] - vec[0];
    for v in vec.windows(2) {
        let before = v[0];
        let after = v[1];
        ans = gcd(ans, after - before);
    }
    println!("{}", ans);
}
