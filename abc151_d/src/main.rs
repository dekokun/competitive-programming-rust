#![allow(non_snake_case)]

use std::collections::VecDeque;
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
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        let row: String = read();
        map.push(row.chars().collect());
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if map[i][j] == '#' {
                continue;
            }
            ans = std::cmp::max(ans, wfs(i, j, &map, H - 1, W - 1));
        }
    }
    println!("{}", ans);
}

fn wfs (i: usize, j: usize, map: &[Vec<char>], i_max: usize, j_max: usize) -> usize {
    let mut depth_map: Vec<Vec<Option<usize>>> = vec![vec![None; j_max + 1]; i_max + 1];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_front((i, j));
    depth_map[i][j] = Some(0);
    let dir = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut max_depth = 0;
    while let Some((i, j)) = queue.pop_back() {
        let depth = depth_map[i][j].unwrap();
        max_depth = std::cmp::max(max_depth, depth);
        for &(di, dj) in &dir {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 || ni > i_max as i32 || nj > j_max as i32 {
                continue;
            }
            let ni: usize = ni as usize;
            let nj: usize = nj as usize;
            if map[ni][nj] == '#' {
                continue;
            }
            if depth_map[ni][nj].is_some() {
                continue;
            }
            queue.push_front((ni, nj));
            depth_map[ni][nj] = Some(depth + 1);
        }

    }
    max_depth
}