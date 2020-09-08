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
    use std::collections::{HashMap, HashSet};
    let mut map: HashMap<usize, usize> = HashMap::new();
    let N: usize = read();
    for i in 1..=N {
        let v: usize = read();
        map.insert(i, v);
    }
    let mut visited: HashSet<usize> = HashSet::new();
    let mut now = 1;
    visited.insert(1);
    let mut ans = 0;
    while let Some(&v) = map.get(&now) {
        ans += 1;
        if v == 2 {
            println!("{}", ans);
            return;
        } else if v == 1 || visited.contains(&v) {
            break;
        }
        now = v;
        visited.insert(v);
    }
    println!("{}", -1);
}
