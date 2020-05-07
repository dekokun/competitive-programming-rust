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
    let H: usize = read();
    let W: usize = read();
    let mut vec: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        let v: Vec<char> = read::<String>().chars().collect();
        assert_eq!(v.len(), W);
        vec.push(v);
    }
    let ans = solve(vec);
    println!("{}", ans);
}

fn solve(mut v: Vec<Vec<char>>) -> usize {
    use std::collections::VecDeque;
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == '#' {
                queue.push_back((i, j, 0));
            }
        }
    }
    let mut max = 0;
    while let Some((i, j, count)) = queue.pop_front() {
        max = std::cmp::max(count, max);
        // 上
        if j != 0 && v[i][j - 1] == '.' {
            v[i][j - 1] = '#';
            queue.push_back((i, j - 1, count + 1));
        }
        // 下
        if j != v[0].len() - 1 && v[i][j + 1] == '.' {
            v[i][j + 1] = '#';
            queue.push_back((i, j + 1, count + 1));
        }
        // 左
        if i != 0 && v[i - 1][j] == '.' {
            v[i - 1][j] = '#';
            queue.push_back((i - 1, j, count + 1));
        }
        // 右
        if i != v.len() - 1 && v[i + 1][j] == '.' {
            v[i + 1][j] = '#';
            queue.push_back((i + 1, j, count + 1));
        }
    }
    max
}
