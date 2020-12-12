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
    let _n: usize = read();
    let k: usize = read();
    let r: usize = read();
    let s: usize = read();
    let p: usize = read();
    let t: Vec<char> = read::<String>().chars().collect();
    let mut t2 = t.clone();
    let mut ans = 0;
    use std::collections::HashMap;
    let mut points: HashMap<char, usize> = HashMap::new();
    points.insert('r', p);
    points.insert('s', r);
    points.insert('p', s);
    for (i, &c) in t.iter().enumerate() {
        if i < k {
            ans += points[&c];
        } else if t2[i - k] == c {
            t2[i] = 'x';
            continue;
        } else {
            ans += points[&c];
        }
    }
    println!("{}", ans);
}
