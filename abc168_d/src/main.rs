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
    let (N, M): (usize, usize) = (read(), read());
    use std::collections::{HashMap, VecDeque};
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..M {
        let a: usize = read();
        let b: usize = read();
        let entry = graph.entry(a).or_insert_with(|| vec![]);
        entry.push(b);
        let entry = graph.entry(b).or_insert_with(|| vec![]);
        entry.push(a);
    }
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_front(1);
    let mut ans: Vec<usize> = vec![0; N + 1];
    while let Some(now) = queue.pop_back() {
        match graph.get(&now) {
            None => {
                continue;
            }
            Some(nexts) => {
                for &next in nexts {
                    if ans[next] != 0 {
                        continue;
                    }
                    ans[next] = now;
                    queue.push_front(next);
                }
            }
        }
    }
    for &v in ans.iter().skip(2) {
        if v == 0 {
            println!("-1");
            return;
        }
    }
    println!("Yes");
    for &v in ans.iter().skip(2) {
        println!("{}", v);
    }
}
