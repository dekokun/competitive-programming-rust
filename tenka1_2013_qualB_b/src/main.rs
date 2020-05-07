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

#[derive(Debug)]
enum Query {
    Push(usize, i32),
    Pop(usize),
    Top,
    Size,
}

#[derive(Debug)]
enum Output {
    FULL,
    EMPTY,
    SAFE,
    Display(i32),
}

fn main() {
    let Q: usize = read();
    let L: usize = read();
    let queries: Vec<Query> = (0..Q)
        .map(|_| {
            let s: String = read();
            match s.as_str() {
                "Push" => Query::Push(read(), read()),
                "Pop" => Query::Pop(read()),
                "Top" => Query::Top,
                "Size" => Query::Size,
                _ => panic!("not implemented query: {}", s),
            }
        })
        .collect();
    let ans = solve(L, queries);
    println!(
        "{}",
        ans.iter()
            .map(|o| match *o {
                Output::FULL => "FULL".to_string(),
                Output::EMPTY => "EMPTY".to_string(),
                Output::SAFE => "SAFE".to_string(),
                Output::Display(a) => a.to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(size: usize, queries: Vec<Query>) -> Vec<Output> {
    let mut stack = vec![];
    let mut ans: Vec<Output> = vec![];
    for q in queries {
        match q {
            Query::Push(count, _val) if size - stack.len() < count => {
                ans.push(Output::FULL);
                break;
            }
            Query::Push(count, val) => {
                for _ in 0..count {
                    stack.push(val)
                }
            }
            Query::Pop(count) if stack.len() < count => {
                ans.push(Output::EMPTY);
                break;
            }
            Query::Pop(count) => {
                for _ in 0..count {
                    stack.pop();
                }
            }
            Query::Top if stack.is_empty() => {
                ans.push(Output::EMPTY);
                break;
            }
            Query::Top => ans.push(Output::Display(*stack.last().unwrap())),
            Query::Size => ans.push(Output::Display(stack.len() as i32)),
        }
    }
    if let Output::Display(_) = *ans.last().unwrap() {
        ans.push(Output::SAFE)
    }
    ans
}
