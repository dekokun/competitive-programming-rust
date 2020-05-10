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
    let M: usize = read();
    let X: usize = read();
    let mut vec = vec![];
    for _ in 0..N {
        let C: usize = read();
        let mut vec2: Vec<usize> = vec![];
        for _ in 0..M {
            vec2.push(read());
        }
        vec.push((C, vec2));
    }
    let ans = dfs(0, 0, vec![0; M], X, &vec);
    println!(
        "{}",
        match ans {
            None => -1,
            Some(a) => a as i32,
        }
    );
}

fn dfs(
    cost: usize,
    idx: usize,
    levels: Vec<usize>,
    target: usize,
    books: &[(usize, Vec<usize>)],
) -> Option<usize> {
    if levels.iter().all(|&v| v >= target) {
        return Some(cost);
    }
    if idx == books.len() {
        return None;
    }
    // skip next book
    let ans1 = dfs(cost, idx + 1, levels.clone(), target, books);
    // don't skip next book
    let next_cost = cost + books[idx].0;
    let mut next_levels = vec![];
    for i in 0..levels.len() {
        next_levels.push(levels[i] + books[idx].1[i]);
    }
    let ans2 = dfs(next_cost, idx + 1, next_levels, target, books);
    match (ans1, ans2) {
        (None, None) => None,
        (None, Some(a)) => Some(a),
        (Some(a), None) => Some(a),
        (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
    }
}
