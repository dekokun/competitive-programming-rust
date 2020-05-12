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

#[derive(Clone)]
enum Edge {
    Start(usize),
    End(usize),
}

fn main() {
    let N: usize = read();
    let M: usize = read();
    let mut vec: Vec<Vec<Edge>> = vec![vec![]; N];
    for i in 0..M {
        let a = read::<usize>() - 1;
        let b = read::<usize>() - 1;
        vec[a].push(Edge::Start(i));
        // 0以下の場合は
        vec[b].push(Edge::End(i));
    }

    use std::collections::HashSet;
    let mut set = HashSet::new();

    let mut ans = 0;
    for actions in vec {
        let mut starts = vec![];
        let mut ends = vec![];
        for v in actions {
            match v {
                Edge::Start(v) => starts.push(v),
                Edge::End(v) => ends.push(v),
            }
        }
        if ends.iter().any(|v| set.contains(v)) {
            set.clear();
            ans += 1;
        }
        for v in starts {
            set.insert(v);
        }
    }
    println!("{}", ans);
}
