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
    let S: String = read();
    use std::collections::HashMap;
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in S.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut one_count = 0;
    let mut two_count = 0;
    for (_, count) in map {
        one_count += count % 2;
        two_count += count / 2;
    }
    if one_count > two_count {
        println!("{}", 1);
        return;
    }
    if one_count == 0 {
        println!("{}", two_count * 2);
        return;
    }
    println!("{}", 1 + (two_count / one_count) * 2);
}
