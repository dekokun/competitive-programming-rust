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
    let n: usize = read();
    let a: Vec<usize> = (0..n).map(|_| read()).collect();
    let q: usize = read();
    let mut sum: usize = a.iter().sum();
    use std::collections::HashMap;
    let mut map: HashMap<usize, usize> = HashMap::new();
    for v in a {
        let entry = map.entry(v).or_insert(0);
        *entry += 1;
    }
    for _ in 0..q {
        let b: usize = read();
        let c: usize = read();
        match map.get(&b) {
            None => {}
            Some(count) => {
                sum -= b * count;
                sum += c * count;
            }
        }
        let entry = map.entry(b).or_insert(0);
        let count = *entry;
        *entry = 0;
        let entry = map.entry(c).or_insert(0);
        *entry += count;
        println!("{}", sum);
    }
}
