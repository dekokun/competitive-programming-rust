#![allow(non_snake_case)]

use std::cmp;
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
    let Q: usize = read();
    let cones: Vec<_> = (0..N)
        .map(|_key| {
            let bottom: usize = read();
            let r: usize = read();
            let top: usize = bottom + read::<usize>();
            (bottom, top, r.pow(2))
        })
        .collect();
    let queries: Vec<_> = (0..Q)
        .map(|_| {
            let a: usize = read();
            let b: usize = read();
            (a, b)
        })
        .collect();
    let ans = solve(cones, queries);
    println!(
        "{}",
        ans.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(cones: Vec<(usize, usize, usize)>, queries: Vec<(usize, usize)>) -> Vec<f64> {
    let max = 2 * 10_usize.pow(4);
    let mut vec: Vec<f64> = vec![0.0; max + 1];
    for i in 0..max + 1 {
        for &(bottom, top, r2) in &cones {
            if i >= top {
                continue;
            }
            let tmp_bottom = cmp::max(i, bottom);
            let origin_height = top - bottom;
            let origin_v = r2 * origin_height;
            let height = top - tmp_bottom;
            vec[i] += origin_v as f64 * (height.pow(3) as f64 / origin_height.pow(3) as f64);
        }
    }
    queries
        .iter()
        .map(|&(a, b)| (vec[a] - vec[b]) * std::f64::consts::PI / 3.0)
        .collect()
}
