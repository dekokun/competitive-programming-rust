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

fn read_vec<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let s: Vec<String> = read_vec();
    let N: usize = read();
    let t: Vec<String> = (0..N).map(|_| read()).collect();
    let mut ans = vec![];
    for word in &s {
        let word: Vec<_> = word.chars().collect();
        let mut matched = false;
        'outer: for ng in &t {
            if ng.len() != word.len() {
                continue;
            }
            for (i, c) in ng.chars().enumerate() {
                if c == '*' {
                    continue;
                }
                if c != word[i] {
                    continue 'outer;
                }
            }
            matched = true;
        }
        if matched {
            ans.push("*".repeat(word.len()))
        } else {
            let word = word.into_iter().collect();
            ans.push(word);
        }
    }
    println!("{}", ans.join(" "));
}
