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
    let N: usize = read();
    let t: usize = read::<usize>() - 1;
    let a: usize = read::<usize>() - 1;
    let mut m: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..N - 1 {
        // 0 indexed
        let a = read::<usize>() - 1;
        let b = read::<usize>() - 1;
        m.entry(a).or_insert_with(|| vec![]).push(b);
        m.entry(b).or_insert_with(|| vec![]).push(a);
    }
    let mut max = 0;
    let t_dist: Vec<usize> = calc_distance(t, &m, N);
    let a_dist: Vec<usize> = calc_distance(a, &m, N);
    for (i, v) in t_dist.iter().enumerate() {
        if *v < a_dist[i] {
            max = std::cmp::max(max, a_dist[i]);
        }
    }
    println!("{}", max - 1);
}

fn calc_distance(origin: usize, map: &HashMap<usize, Vec<usize>>, max: usize) -> Vec<usize> {
    let mut vec = vec![0; max];
    dfs(origin, origin, &map, 0, &mut vec);
    vec
}
fn dfs(
    now: usize,
    parent: usize,
    map: &HashMap<usize, Vec<usize>>,
    depth: usize,
    result: &mut Vec<usize>,
) {
    result[now] = depth;
    for &child in &map[&now] {
        if child == parent {
            continue;
        }
        dfs(child, now, map, depth + 1, result)
    }
}
