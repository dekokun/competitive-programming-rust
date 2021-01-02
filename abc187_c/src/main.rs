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

use std::collections::HashSet;
fn main() {
    let n: usize = read();
    let mut bang: HashSet<String> = HashSet::new();
    let mut normal: HashSet<String> = HashSet::new();
    for _ in 0..n {
        let s: String = read();
        if s.chars().next().unwrap() == '!' {
            let s: String = s.chars().skip(1).collect();
            if normal.contains(&s) {
                println!("{}", s);
                return
            }
            bang.insert(s);
        } else {
            if bang.contains(&s) {
                println!("{}", s);
                return
            }
            normal.insert(s);
        }
    }
    println!("satisfiable");
}
