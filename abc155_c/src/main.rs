#![allow(non_snake_case)]
use std::collections::HashMap;
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
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut rev_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut max = 0;
    for _ in 0..N {
        let str: String = read();
        let entry = map.entry(str.clone()).or_insert(0);
        *entry += 1;
        let entry2 = rev_map.entry(*entry).or_insert(vec![]);
        (*entry2).push(str.clone());
        max = std::cmp::max(*entry, max);
    }
    let mut vec = rev_map.get(&max).unwrap().clone();
    vec.sort();
    for v in vec {
        println!("{}", v);
    }
}
