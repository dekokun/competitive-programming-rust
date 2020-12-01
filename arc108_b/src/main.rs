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
    let _n: usize = read();
    let s: String = read();
    let mut vec = vec![];
    'outer: for c in s.chars() {
        vec.push(c);
        if c == 'x' && vec.len() >= 3 {
            if vec[vec.len() - 1] == 'x' && vec[vec.len() - 2] == 'o' && vec[vec.len() - 3] == 'f' {
                vec.pop();
                vec.pop();
                vec.pop();
            }
        }
    }
    // dbg!(&vec);
    println!("{}", vec.len());
}