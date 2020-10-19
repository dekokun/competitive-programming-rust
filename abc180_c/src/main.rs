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
    let n: u64 = read();
    use std::collections::HashSet;
    let mut ans: HashSet<u64> = HashSet::new();
    for i in 1..=(n as f64).sqrt() as u64 + 1 {
        if n % i == 0 {
            ans.insert(i);
            ans.insert(n / i);
        }
    }
    let mut ans: Vec<_> = ans.into_iter().collect();
    ans.sort();
    for v in ans {
        println!("{}", v);
    }
}
