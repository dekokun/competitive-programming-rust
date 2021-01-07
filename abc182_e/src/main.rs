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
    let h: usize = read();
    let w: usize = read();
    let n: usize = read();
    let m: usize = read();
    let mut bulbs = vec![];
    for _ in 0..n {
        let a: usize = read();
        let b: usize = read();
        bulbs.push((a - 1, b - 1));
    }
    let mut blocks = vec![vec![false; w]; h];
    for _ in 0..m {
        let c: usize = read();
        let d: usize = read();
        blocks[c - 1][d - 1] = true;
    }
    let mut illuminated_d = vec![vec![false; w]; h];
    for &(a, b) in &bulbs {
        if illuminated_d[a as usize][b as usize] {
            continue;
        }
        for &(d, r) in &[(1, 0), (-1, 0)] {
            for dist in 0..=1500 {
                let a: i32 = a as i32;
                let b: i32 = b as i32;
                let next_a = a + d * dist;
                let next_b = b + r * dist;
                if next_a >= h as i32
                    || next_b >= w as i32
                    || next_a < 0
                    || next_b < 0
                    || blocks[next_a as usize][next_b as usize]
                {
                    break;
                }
                illuminated_d[next_a as usize][next_b as usize] = true;
            }
        }
    }
    let mut illuminated_r = vec![vec![false; w]; h];
    for &(a, b) in &bulbs {
        if illuminated_r[a as usize][b as usize] {
            continue;
        }
        for &(d, r) in &[(0, 1), (0, -1)] {
            for dist in 0..=1500 {
                let a: i32 = a as i32;
                let b: i32 = b as i32;
                let next_a = a + d * dist;
                let next_b = b + r * dist;
                if next_a >= h as i32
                    || next_b >= w as i32
                    || next_a < 0
                    || next_b < 0
                    || blocks[next_a as usize][next_b as usize]
                {
                    break;
                }
                illuminated_r[next_a as usize][next_b as usize] = true;
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if illuminated_d[i][j] || illuminated_r[i][j] {
                ans += 1;
            }
        }
    }
    println!(
        "{}",
        ans
    );
}
