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
    let mut graph: Vec<Vec<usize>> = vec![vec![]; N];
    for _ in 0..M {
        let a: usize = read::<usize>() - 1;
        let b: usize = read::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
    }
    use std::collections::VecDeque;
    let mut queue: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    queue.push_front((0, vec![0]));
    let mut ans = 0;
    while let Some((num, path)) = queue.pop_back() {
        if path.len() == N {
            ans += 1;
        }
        for &next in &graph[num] {
            if !path.contains(&next) {
                let mut path = path.clone();
                path.push(next);
                queue.push_back((next, path));
            }
        }
    }
    println!("{}", ans);
}
