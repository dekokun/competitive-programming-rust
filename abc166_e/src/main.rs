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
    let mut starts: HashMap<i32, usize> = HashMap::new();
    let mut ends: HashMap<i32, usize> = HashMap::new();
    for i in 0..N {
        let a: i32 = read();
        let se = starts.entry(i as i32 - a).or_insert(0);
        *se += 1;
        let ee = ends.entry(i as i32 + a).or_insert(0);
        *ee += 1;
    }
    let mut ans = 0;
    for (k, v) in ends {
        match starts.get(&(k)) {
            Some(a) => {
                ans += a * v;
            }
            None => continue,
        }
    }
    println!("{}", ans)
}
