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
    let H: isize = read();
    let W: isize = read();
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        map.push(read::<String>().chars().collect());
    }
    use std::collections::VecDeque;
    let mut q: VecDeque<(isize, isize, usize)> = VecDeque::new();
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut costs: Vec<Vec<Option<usize>>> = vec![vec![None; W as usize]; H as usize];

    let mut min_cost = None;
    q.push_back((0, 0, 1));
    costs[0][0] = Some(1);
    while let Some((i, j, cost)) = q.pop_front() {
        if i == H - 1 && j == W - 1 {
            min_cost = Some(cost);
            break;
        }
        for &(di, dj) in &directions {
            let ni = i + di;
            let nj = j + dj;
            if ni < 0 || H <= ni {
                continue;
            }
            if nj < 0 || W <= nj {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if costs[ni][nj].is_some() {
                continue;
            }
            if map[ni][nj] == '#' {
                continue;
            }
            q.push_back((ni as isize, nj as isize, cost + 1));
            costs[ni][nj] = Some(cost + 1);
        }
    }
    match min_cost {
        None => println!("-1"),
        Some(min_cost) => {
            let mut white_count = 0;
            for row in map {
                for cell in row {
                    if cell == '.' {
                        white_count += 1;
                    }
                }
            }
            println!("{}", white_count - min_cost);
        }
    }
}
