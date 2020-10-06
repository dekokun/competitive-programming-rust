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
    let mut strings: Vec<Vec<char>> = vec![vec![' '; 4]; 4];
    for i in 0..4 {
        let mut line = "".to_owned();
        for _ in 0..4 {
            line.push(read());
        }
        strings[4 - i - 1] = line.chars().rev().collect();
    }
    dbg!(&strings);
    for s in strings {
        println!(
            "{}",
            s.into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
