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
    let mut set: std::collections::HashSet<String> = std::collections::HashSet::new();
    let vec: Vec<String> = (0..N).map(|_| read()).collect();
    let mut before_last = vec[0].chars().next().unwrap();
    let mut takahashi_win = false;
    for v in vec {
        if v.chars().next().unwrap() != before_last {
            println!("{}", if takahashi_win { "WIN" } else { "LOSE" });
            return;
        }
        if !set.insert(v.clone()) {
            println!("{}", if takahashi_win { "WIN" } else { "LOSE" });
            return;
        };
        takahashi_win = !takahashi_win;
        before_last = v.chars().last().unwrap();
    }
    println!("DRAW");
}
