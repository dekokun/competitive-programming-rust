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
    let mut map: HashMap<usize, usize> = HashMap::new();
    for _ in 0..N {
        let entry = map.entry(read()).or_insert(0);
        *entry += 1;
    }
    let mut ans = map.len();
    let mut parity = 0;
    for v in map.values() {
        let v = if v % 2 == 0 { 2 } else { 1 };
        if v == 2 {
            if parity != 0 {
                parity = 0;
            } else {
                parity += 1
            }
        }
    }
    ans -= parity;
    println!("{}", ans);
}
