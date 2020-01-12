#![allow(non_snake_case)]

use std::io::{stdin, Read};
use std::str::FromStr;
use std::collections::HashMap;
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
    let M: usize = read();
    let mut map: HashMap<usize, Vec<bool>> = HashMap::new();
    for _ in 0..M {
        let i: usize = read::<usize>();
        let mut entry = map.entry(i).or_insert(vec![]);
        (*entry).push(read::<String>() == "AC");
    }
    let mut correct: usize = 0;
    let mut penalty: usize = 0;
    for (_, v) in map {
        let mut penalty_tmp: usize = 0;
        for b in v {
            if b {
                correct += 1;
                penalty += penalty_tmp;
                break;
            } else {
                penalty_tmp += 1;
            }
        }
    }
    println!("{} {}", correct, penalty);
}
