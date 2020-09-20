#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet, VecDeque};
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
    let ps: Vec<usize> = (0..N).map(|_| read::<usize>() - 1).collect();
    let ms: Vec<(usize, usize)> = (0..M)
        .map(|_| (read::<usize>() - 1, read::<usize>() - 1))
        .collect();
    println!("{}", solve(ps, ms) + 1)
}

fn solve(ps: Vec<usize>, ms: Vec<(usize, usize)>) -> usize {
    // 1. msがつなぐグループを作る(i と値両方)
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for (x, y) in ms {
        let entry = graph.entry(x).or_insert_with(|| vec![]);
        entry.push(y);
        let entry = graph.entry(y).or_insert_with(|| vec![]);
        entry.push(x);
    }
    let mut visited = vec![false; ps.len()];
    let mut ans = 0;
    for i in 0..ps.len() {
        if visited[i] {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_front(i);
        visited[i] = true;
        let mut indexes = HashSet::new();
        let mut values = HashSet::new();
        indexes.insert(i);
        values.insert(ps[i]);
        while let Some(v) = queue.pop_back() {
            match graph.get(&v) {
                None => {
                    continue;
                }
                Some(vec) => {
                    for &next in vec {
                        if visited[next] {
                            continue;
                        }
                        queue.push_front(next);
                        visited[next] = true;
                        indexes.insert(next);
                        values.insert(ps[next]);
                    }
                }
            }
        }
        for i in indexes {
            if values.contains(&i) {
                ans += 1;
            }
        }
    }
    ans - 1
}
