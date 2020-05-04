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
    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(0, 1);
    for _ in 0..N {
        let entry = map.entry(read()).or_insert(0);
        *entry += 1;
        if *entry >= 3 {
            println!("0");
            return;
        }
    }
    let mut vec: Vec<(usize, usize)> = vec![];
    for v in map {
        vec.push(v);
    }
    let mins = dfs(0, &vec, vec![], vec![]);
    println!("{}", mins.iter().max().unwrap());
}

fn dfs(
    depth: usize,
    vec: &[(usize, usize)],
    now_times: Vec<usize>,
    mins: Vec<usize>,
) -> Vec<usize> {
    if depth == vec.len() {
        let mut now_times = now_times;
        now_times.sort();
        now_times.push(24);
        let min = now_times
            .windows(2)
            .fold(1000, |acc, v| std::cmp::min(acc, (v[1] - v[0]) % 24));
        let mut mins = mins;
        mins.push(min);
        return mins;
    }
    let v = vec[depth].0;
    let count = vec[depth].1;
    if count == 2 {
        let mut now_times = now_times;
        now_times.push(v);
        now_times.push(24 - v);
        dfs(depth + 1, vec, now_times, mins)
    } else {
        let mut now_times_mut = now_times.clone();
        now_times_mut.push(v);
        let mins1 = dfs(depth + 1, vec, now_times_mut, mins.clone());
        let mut now_times_mut = now_times;
        now_times_mut.push(24 - v);
        let mins2 = dfs(depth + 1, vec, now_times_mut, mins);
        [mins1, mins2].concat()
    }
}
