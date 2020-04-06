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
    use std::collections::HashMap;
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..N {
        let v: usize = read();
        let entry = map.entry(v).or_insert(vec![]);
        entry.push(v);
    }
    let mut ans = 0;
    for (key, value) in map {
        use std::cmp::Ordering;
        match key.cmp(&value.len()) {
            Ordering::Equal => continue,
            Ordering::Less => ans += value.len() - key,
            Ordering::Greater => ans += value.len(),
        }
    }
    println!("{}", ans);
}
