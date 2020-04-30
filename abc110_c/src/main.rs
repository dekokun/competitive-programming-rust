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
    let S: Vec<char> = read::<String>().chars().collect();
    let T: Vec<char> = read::<String>().chars().collect();
    use std::collections::HashMap;
    let mut map: HashMap<char, char> = HashMap::new();
    let mut rev_map: HashMap<char, char> = HashMap::new();
    for i in 0..S.len() {
        if let Some(&t) = map.get(&S[i]) {
            if t != T[i] {
                println!("No");
                return;
            }
        } else {
            map.insert(S[i], T[i]);
        }
        if let Some(&s) = rev_map.get(&T[i]) {
            if s != S[i] {
                println!("No");
                return;
            }
        } else {
            rev_map.insert(T[i], S[i]);
        }
    }
    println!("Yes");
}
