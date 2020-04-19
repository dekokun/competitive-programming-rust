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
    let _N: usize = read();
    let S: String = read();
    let mut unique = vec![];
    for c in S.chars() {
        match unique.pop() {
            None => unique.push(c),
            Some(before) => {
                if before != c {
                    unique.push(before);
                    unique.push(c);
                }
            }
        }
    }
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for v in unique {
        if set.contains(&v) {
            set.remove(&v);
        } else {
            set.insert(v);
        }
    }
    println!("{}", set.len());
}
