#![allow(non_snake_case)]

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
        ($($a:expr),* $(,)*) => {
            #[cfg(debug_assertions)]
            eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
        };
    }

use std::str::FromStr;
use std::{
    collections::VecDeque,
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

fn main() {
    let n: usize = read();
    let inf = std::i64::MAX;
    let mut graph = vec![(0, vec![]); n];
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let a: usize = read();
        let b: usize = read();
        let a = a - 1;
        let b = b - 1;

        edges.push((a, b));

        graph[a].1.push(b);
        graph[b].1.push(a);
    }
    // 木が0始まりだとして、各ノードの深さを記録していく
    let mut depth = vec![inf; n];
    depth[0] = 0;
    let mut q = VecDeque::new();
    q.push_front((0, 0));
    while let Some((pos, cost)) = q.pop_back() {
        for &next in &graph[pos].1 {
            if depth[next] != inf {
                continue;
            }
            depth[next] = cost + 1;
            q.push_front((next, cost + 1));
        }
    }

    let q: usize = read();
    for _ in 0..q {
        let t: usize = read();
        let e: usize = read();
        let x: i64 = read();
        let e = e - 1;
        let (a, b) = if t == 1 {
            (edges[e].0, edges[e].1)
        } else {
            (edges[e].1, edges[e].0)
        };
        if depth[a] < depth[b] {
            graph[0].0 += x;
            graph[b].0 -= x;
        } else {
            graph[a].0 += x;
        }
    }
    let mut q = VecDeque::new();
    q.push_front((0, graph[0].0));
    let mut ans = vec![inf; n];
    ans[0] = graph[0].0;
    debug!(graph);
    while let Some((pos, cost)) = q.pop_back() {
        for &next in &graph[pos].1 {
            if ans[next] != inf {
                continue;
            }
            let cost = cost + graph[next].0;
            ans[next] = cost;
            q.push_front((next, cost));
        }
    }
    for v in ans {
        println!("{}", v)
    }
}
