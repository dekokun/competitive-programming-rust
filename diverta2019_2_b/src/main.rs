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
    let mut p_q: HashMap<(i32, i32), usize> = HashMap::new();
    let N: usize = read();
    let mut input: Vec<(i32, i32)> = vec![];
    for _ in 0..N {
        input.push((read(), read()));
    }
    for &(x1, y1) in &input {
        for &(x2, y2) in &input {
            if x1 == x2 && y1 == y2 {
                continue;
            }
            let entry = p_q.entry((x1 - x2, y1 - y2)).or_insert(0);
            *entry += 1;
        }
    }
    let mut ans = 0;
    for (_key, v) in p_q {
        ans = std::cmp::max(v, ans);
    }
    println!("{}", N - ans);
}
