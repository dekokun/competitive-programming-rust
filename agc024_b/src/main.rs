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
    let mut vec: Vec<usize> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    let mut now_val = 0;
    for &v in &vec {
        if v == now_val + 1 {
            now_val += 1;
        }
    }
    let mut ans_candidates = vec![];
    ans_candidates.push(N - now_val);
    vec.reverse();
    let mut now_val = N + 1;
    for &v in &vec {
        if v == now_val - 1 {
            now_val -= 1;
        }
    }
    ans_candidates.push(now_val - 1);
    println!("{}", std::cmp::min(ans_candidates[0], ans_candidates[1]));
}
