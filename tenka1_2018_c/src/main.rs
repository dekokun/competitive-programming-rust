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
    let mut vec: Vec<usize> = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec.sort();
    let mut vec2 = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            vec2.push(vec[i / 2]);
        } else {
            vec2.push(vec[n - 1 - i / 2]);
        }
    }
    dbg!(&vec2);
    let mut ans = 0;
    for v in vec2.windows(2) {
        ans += std::cmp::max(v[0], v[1]) - std::cmp::min(v[0], v[1]);
    }
    println!("{}", ans)
}
