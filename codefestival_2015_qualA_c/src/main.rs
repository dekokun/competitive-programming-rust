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
    let T: usize = read();
    let mut ab: Vec<(usize, usize)> = vec![];
    for _ in 0..N {
        ab.push((read(), read()))
    }
    if ab.iter().map(|(_a, b)| *b).sum::<usize>() > T {
        println!("-1");
        return;
    }
    ab.sort_by_key(|(a, b)| a - b);
    let a_sum = ab.iter().map(|(a, _b)| *a).sum::<usize>();
    let mut sum = a_sum;
    let mut ans = 0;
    while sum > T {
        let (a, b) = ab.pop().unwrap();
        sum -= a - b;
        ans += 1;
    }
    println!("{}", ans)
}
