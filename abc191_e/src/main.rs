#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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
    (1..=n).map(|i| bfs(&graph, i)).collect()
}

fn bfs(graph: &Vec<Vec<(usize, i64)>>, start: usize) -> i64 {
    // (pos, cost)
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));
    let mut costs = vec![];
    while let Some((now, cost)) = queue.pop_back() {
        if now == start && cost != 0 {
            costs.push(cost);
        }
        for &v in &graph[now] {
            queue.push_front((v.0, cost + v.1));
        }
    }
    if costs.is_empty() {
        -1
    } else if costs.iter().all(|&v| v == -1) {
        -1
    } else {
        costs.into_iter().filter(|&v| v >= 0).min().unwrap()
    }
}

// fn solve_dfs(n: usize, _m: usize, abc: Vec<(usize, usize, i64)>) -> Vec<i64> {
//     let mut graph = vec![vec![]; n + 1];
//     for (a, b, c) in abc {
//         graph[a].push((b, c));
//     }
//     debug!(graph);

//     let set: HashSet<usize> = HashSet::new();
//     (1..=n).map(|i| dfs(&graph, set.clone(), 0, i, i)).collect()
// }

// fn dfs(
//     graph: &Vec<Vec<(usize, i64)>>,
//     visited: HashSet<usize>,
//     cost: i64,
//     now: usize,
//     start: usize,
// ) -> i64 {
//     if now == start && !visited.is_empty() {
//         return cost;
//     }
//     let mut values = vec![];
//     for &v in &graph[now] {
//         if visited.contains(&v.0) {
//             continue;
//         }
//         let mut visited = visited.clone();
//         visited.insert(v.0);
//         values.push(dfs(graph, visited, cost + v.1, v.0, start));
//     }
//     if values.is_empty() {
//         -1
//     } else if values.iter().all(|&v| v == -1) {
//         -1
//     } else {
//         values.into_iter().filter(|&v| v >= 0).min().unwrap()
//     }
// }
