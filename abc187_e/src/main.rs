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

use std::collections::HashMap;

fn main() {
    let n: usize = read();
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut edges = vec![];
    for _ in 0..n - 1 {
        let a = read();
        let b = read();

        edges.push((a, b));

        let entry = graph.entry(a).or_insert_with(|| vec![]);
        entry.push(b);
        let entry = graph.entry(b).or_insert_with(|| vec![]);
        entry.push(a);
    }
    // 木が1始まりだとして、深さ優先探索をしつつvectorに落とす
    let mut vec = vec![];
    vec.push((1, 1));

    let tree_vec = dfs(1, 1, &graph);
    // dbg!(tree_vec);
    let mut imos = vec![0; tree_vec.len()];
    let q: usize = read();
    for _ in 0..q {
        let t: usize = read();
        let e: usize = read();
        let x: usize = read();
    }
}

fn dfs(before: usize, next: usize, graph: &HashMap<usize, Vec<usize>>) -> Vec<(usize, usize)> {
    let mut ans = vec![(next, 0)];
    if let Some(a) = graph.get(&next) {
        for &v in a {
            if v == before {
                continue;
            }
            let mut a = dfs(next, v, graph);
            ans.append(&mut a);
        }
    }
    ans[0] = (next, ans.len());
    ans
}
