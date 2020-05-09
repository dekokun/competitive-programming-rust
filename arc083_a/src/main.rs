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
    let A: usize = read();
    let B: usize = read();
    let C: usize = read();
    let D: usize = read();
    let E: usize = read();
    let F: usize = read();
    let ans = solve((A, B), (C, D), E, F);
    println!("{} {}", ans.0, ans.1);
}

fn solve(
    waters: (usize, usize),
    salts: (usize, usize),
    disolve_max: usize,
    beaker_max: usize,
) -> (usize, usize) {
    use std::collections::VecDeque;
    let mut water_queue = VecDeque::new();
    let mut water_map: Vec<bool> = vec![false; beaker_max + 1];
    water_queue.push_front(0_usize);
    // 水が0はありえない
    water_map[0] = false;

    while let Some(g) = water_queue.pop_back() {
        let next_0 = g + waters.0 * 100;
        if next_0 <= beaker_max && !water_map[next_0] {
            water_queue.push_front(next_0);
            water_map[next_0] = true;
        }
        let next_1 = g + waters.1 * 100;
        if next_1 <= beaker_max && !water_map[next_1] {
            water_queue.push_front(next_1);
            water_map[next_1] = true;
        }
    }

    let mut salt_queue = VecDeque::new();
    let mut salt_map: Vec<bool> = vec![false; beaker_max + 1];
    salt_queue.push_front(0_usize);
    salt_map[0] = true;
    while let Some(g) = salt_queue.pop_back() {
        let next_0 = g + salts.0;
        if next_0 <= beaker_max && !salt_map[next_0] {
            salt_queue.push_front(next_0);
            salt_map[next_0] = true;
        }
        let next_1 = g + salts.1;
        if next_1 <= beaker_max && !salt_map[next_1] {
            salt_queue.push_front(next_1);
            salt_map[next_1] = true;
        }
    }
    let salt_possibles: Vec<usize> = salt_map
        .into_iter()
        .enumerate()
        .filter(|&v| v.1)
        .map(|v| v.0)
        .collect();
    let water_possibles: Vec<usize> = water_map
        .into_iter()
        .enumerate()
        .filter(|&v| v.1)
        .map(|v| v.0)
        .collect();
    let mut max_density = 0.0;
    let mut tuple = (100 * waters.0, 0);
    for &s in &salt_possibles {
        for &w in &water_possibles {
            if s + w > beaker_max {
                continue;
            }
            if s + w == 0 {
                continue;
            }
            if disolve_max * (w / 100) < s {
                continue;
            }
            let density = s as f64 / (w + s) as f64;
            if max_density < density {
                max_density = density;
                tuple = (s + w, s);
            }
        }
    }
    tuple
}
