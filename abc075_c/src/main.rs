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
    let mut vertexes: Vec<(usize, usize)> = vec![];
    for _ in 0..M {
        vertexes.push((read(), read()));
    }
    let mut ans = 0;
    'outer: for i in 0..M {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (j, &(a, b)) in vertexes.iter().enumerate() {
            if j == i {
                continue;
            }
            let entry = graph.entry(a).or_insert_with(HashSet::new);
            entry.insert(b);
            let entry = graph.entry(b).or_insert_with(HashSet::new);
            entry.insert(a);
        }
        for start in 1..=N {
            for goal in 1..=N {
                if start == goal {
                    continue;
                }
                let mut visited = HashSet::new();
                let mut queue = VecDeque::new();
                visited.insert(start);
                queue.push_front(start);
                let mut goaled = false;
                while let Some(now) = queue.pop_back() {
                    if now == goal {
                        goaled = true;
                        break;
                    }
                    let nexts = graph.get(&now);
                    match nexts {
                        None => break,
                        Some(nexts) => {
                            for &next in nexts {
                                if visited.contains(&next) {
                                    continue;
                                }
                                queue.push_front(next);
                                visited.insert(next);
                            }
                        }
                    }
                }
                if !goaled {
                    ans += 1;
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", ans);
}
