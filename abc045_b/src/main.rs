#![allow(non_snake_case)]

use std::collections::HashMap;

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
    let mut S: HashMap<char, Vec<char>> = HashMap::new();
    let SA: String = read();
    let SB: String = read();
    let SC: String = read();
    S.insert('a', SA.chars().rev().collect());
    S.insert('b', SB.chars().rev().collect());
    S.insert('c', SC.chars().rev().collect());
    let mut turn = 'a';
    loop {
        if S.get(&turn).unwrap().is_empty() {
            println!(
                "{}",
                match turn {
                    'a' => 'A',
                    'b' => 'B',
                    'c' => 'C',
                    _ => panic!(),
                }
            );
            return;
        } else {
            let entry = S.entry(turn).or_insert_with(|| vec![]);
            turn = entry.pop().unwrap();
        }
    }
}
