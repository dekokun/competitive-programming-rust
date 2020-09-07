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
    let s: String = read();
    let k: usize = read();
    if k > s.len() {
        println!("0");
        return;
    }
    use std::collections::HashSet;
    let mut set: HashSet<String> = HashSet::new();
    for s2 in s.chars().collect::<Vec<_>>().windows(k) {
        let s2: String = s2.into_iter().collect();
        set.insert(s2);
    }
    println!("{}", set.len());
}
