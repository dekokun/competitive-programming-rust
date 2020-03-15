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
    use std::collections::HashMap;
    let mut map: HashMap<char, usize> = HashMap::new();
    map.insert('b', 1);
    map.insert('c', 1);
    map.insert('d', 2);
    map.insert('w', 2);
    map.insert('t', 3);
    map.insert('j', 3);
    map.insert('f', 4);
    map.insert('q', 4);
    map.insert('l', 5);
    map.insert('v', 5);
    map.insert('s', 6);
    map.insert('x', 6);
    map.insert('p', 7);
    map.insert('m', 7);
    map.insert('h', 8);
    map.insert('k', 8);
    map.insert('n', 9);
    map.insert('g', 9);
    map.insert('z', 0);
    map.insert('r', 0);
    let N: usize = read();
    let mut ans_vec = vec![];
    for _ in 0..N {
        let word: String = read::<String>().to_lowercase();
        let transformed = word.chars().fold("".to_string(), |acc, c| {
            let st = match map.get(&c) {
                None => "".to_string(),
                Some(c) => c.to_string(),
            };
            acc + &st
        });
        if transformed != "" {
            ans_vec.push(transformed);
        }
    }
    println!("{}", ans_vec.join(" "));
}
