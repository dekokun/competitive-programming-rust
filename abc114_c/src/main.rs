#![allow(non_snake_case)]

use std::{
    collections::HashSet,
    io::{stdin, Read},
};
use std::{collections::VecDeque, str::FromStr};
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
    println!("{}", solve(read()));
}

fn solve(n: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(0);
    let mut ans = 0;
    while let Some(v) = queue.pop_back() {
        if v > n {
            continue;
        }

        let mut tmp = HashSet::new();
        for c in v.to_string().chars() {
            tmp.insert(c);
        }
        if tmp.len() == 3 {
            ans += 1;
        }
        for &new in &[7, 5, 3] {
            let next = v * 10 + new;
            queue.push_front(next);
        }
    }
    ans
}
