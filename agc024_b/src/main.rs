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
    use std::collections::HashMap;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for v in vec {
        {
            let entry = map.entry(v - 1).or_insert(0);
            *entry += 1;
        }
        let count = map.remove(&(v - 1)).unwrap();
        map.insert(v, count);
    }
    let mut max = 0;
    for (_i, v) in map {
        max = std::cmp::max(max, v);
    }
    println!("{}", N - max);
}
