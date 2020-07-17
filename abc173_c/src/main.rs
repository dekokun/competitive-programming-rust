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
    let K: usize = read();
    let mut vec: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        vec.push(read::<String>().chars().collect())
    }
    let ans = dfs(0, vec![], H, W, K, &vec);
    println!("{}", ans)
}

fn dfs(now: usize, which: Vec<bool>, h: usize, w: usize, k: usize, map: &Vec<Vec<char>>) -> usize {
    if now == h + w {
        let hs = &which[0..h];
        let ws = &which[h..h + w];
        let mut count = 0;
        for (i, row) in map.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                // true: 塗っているなので、黒くない
                if hs[i] || ws[j] {
                    continue;
                }
                if cell == '#' {
                    count += 1;
                }
            }
        }
        return if count == k { 1 } else { 0 };
    }
    let mut w1 = which.clone();
    w1.push(true);
    let a = dfs(now + 1, w1, h, w, k, map);
    let mut w1 = which;
    w1.push(false);
    let b = dfs(now + 1, w1, h, w, k, map);
    a + b
}
