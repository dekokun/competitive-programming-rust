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
    let T: usize = read();
    for t in 1..T + 1 {
        println!("Case #{}: {}", t, solve());
    }
}

fn solve() -> String {
    let N: usize = read();
    let mut tasks = vec![];
    for index in 0..N {
        let start: usize = read();
        let end: usize = read();
        tasks.push((index, start, end));
    }
    use std::collections::HashSet;
    let mut timeRange = vec![vec![]; 60 * 24 + 1];
    for &task in &tasks {
        timeRange[task.1].push(("start", task.0));
        timeRange[task.2].push(("end", task.0));
    }
    let mut taskSet = HashSet::new();
    let mut ans_vec = vec!['0'; N];
    let mut Cameron = false;
    for tasks in timeRange {
        let mut starts = vec![];
        for task in tasks {
            match task.0 {
                "start" => {
                    taskSet.insert(task.1);
                    starts.push(task.1);
                }
                "end" => {
                    taskSet.remove(&task.1);
                    match ans_vec[task.1] {
                        'C' => Cameron = false,
                        'J' => {}
                        _ => panic!("neither cameron, nor jessie"),
                    }
                }
                _ => panic!("strange value in task"),
            }
        }
        if taskSet.len() >= 3 {
            return "IMPOSSIBLE".into();
        }
        if starts.is_empty() {
            continue;
        }
        // その時に始まったタスクのみ。適当に選ぶ
        if starts.len() == 2 {
            ans_vec[starts[0]] = 'C';
            ans_vec[starts[1]] = 'J';
            Cameron = true;
        }
        // 基本的にCameronにやらせるがCameronが手一杯ならJamieに
        if starts.len() == 1 {
            if !Cameron {
                ans_vec[starts[0]] = 'C';
                Cameron = true;
            } else {
                ans_vec[starts[0]] = 'J';
            }
        }
    }
    ans_vec.iter().collect()
}
