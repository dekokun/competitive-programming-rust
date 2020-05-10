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
    let K: i64 = read();
    let vec: Vec<usize> = (0..N).map(|_| read::<usize>() - 1).collect();
    use std::collections::HashMap;
    let mut set = HashMap::new();
    let mut now = 0;
    let mut vec2 = vec![];
    vec2.push(now);
    set.insert(now, 0);
    loop {
        now = vec[now];
        match set.get(&now) {
            None => {
                vec2.push(now);
                set.insert(now, vec2.len() - 1);
            }
            Some(_) => break,
        }
    }
    let &loop_first_idx = set.get(&now).unwrap();
    if K < loop_first_idx as i64 {
        println!("{}", vec2[K as usize] + 1);
        return;
    }
    let rem = (K - loop_first_idx as i64) % (set.len() - loop_first_idx) as i64;
    println!("{}", vec2[rem as usize + loop_first_idx] + 1);
}
