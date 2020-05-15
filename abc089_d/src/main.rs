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
    let D: usize = read();
    let mut board: Vec<Vec<usize>> = vec![];
    for _ in 0..H {
        let mut vec: Vec<usize> = vec![];
        for _ in 0..W {
            let v = read::<usize>() - 1;
            vec.push(v);
        }
        board.push(vec);
    }
    let Q: usize = read();
    let queries: Vec<(usize, usize)> = (0..Q)
        .map(|_| (read::<usize>() - 1, read::<usize>() - 1))
        .collect();
    let ans = solve(D, &board, &queries);
    println!(
        "{}",
        ans.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(d: usize, board: &[Vec<usize>], queries: &[(usize, usize)]) -> Vec<usize> {
    let mut rev_board: Vec<(usize, usize)> = vec![(0, 0); board.len() * board[0].len()];
    for (i, row) in board.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            rev_board[v] = (i, j);
        }
    }
    let mut scores = vec![vec![0]; d];
    for i in d..rev_board.len() {
        let rem = i % d;
        let before = rev_board[i - d];
        let now = rev_board[i];
        let score = (before.0 as isize - now.0 as isize).abs() as usize
            + (before.1 as isize - now.1 as isize).abs() as usize;
        let last = scores[rem].last().unwrap().clone();
        scores[rem].push(score + last);
    }
    queries
        .iter()
        .map(|&(start, end)| {
            let rem = start % d;
            scores[rem][end / d] - scores[rem][start / d]
        })
        .collect()
}
