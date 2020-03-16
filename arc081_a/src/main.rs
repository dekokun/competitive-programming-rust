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
    use std::collections::HashMap;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for _ in 0..N {
        let val = read();
        let entry = map.entry(val).or_insert(0);
        *entry += 1;
        if *entry % 2 == 0 {
            vec.push(val);
        }
    }
    if vec.len() < 2 {
        println!("0");
    } else {
        vec.sort();
        println!("{}", vec[vec.len() - 1] * vec[vec.len() - 2]);
    }
}
