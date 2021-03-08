#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::{cmp::Reverse, str::FromStr};
use std::{
    collections::BinaryHeap,
    io::{stdin, Read},
};
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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}

fn main() {
    let n: usize = read();
    let m = read();
    println!(
        "{}",
        solve(n, m, (0..m).map(|_| (read(), read(), read())).collect())
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(n: usize, _m: usize, abc: Vec<(usize, usize, i64)>) -> Vec<i64> {
    let mut graph = vec![vec![]; n + 1];
    for (a, b, c) in abc {
        graph[a].push((b, c));
    }
    (1..=n).map(|i| dijkstra(&graph, i, n)).collect()
}

fn dijkstra(graph: &Vec<Vec<(usize, i64)>>, start: usize, n: usize) -> i64 {
    // (cost, pos)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    let mut score = vec![1_000_000_000; n + 1];
    while let Some(Reverse((cost, now))) = heap.pop() {
        if now == start && cost != 0 {
            return cost;
        }
        for &v in &graph[now] {
            if cost + v.1 < score[v.0] {
                heap.push(Reverse((cost + v.1, v.0)));
                score[v.0] = cost + v.1;
            }
        }
    }
    -1
}
