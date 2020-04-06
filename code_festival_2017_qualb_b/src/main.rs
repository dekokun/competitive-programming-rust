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
    let mut candidates = HashMap::new();
    for _ in 0..N {
        let v: usize = read();
        let entry = candidates.entry(v).or_insert(0);
        *entry += 1;
    }
    let M: usize = read();
    let mut problems = HashMap::new();
    for _ in 0..M {
        let v: usize = read();
        let entry = problems.entry(v).or_insert(0);
        *entry += 1;
    }
    for (p, count) in problems {
        match candidates.get(&p) {
            None => {
                println!("NO");
                return;
            }
            Some(a) => {
                if *a < count {
                    println!("NO");
                    return;
                }
            }
        }
    }
    println!("YES");
}
