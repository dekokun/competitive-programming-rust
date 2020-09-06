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
    let N: usize = read();
    let mut nums: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    for i in 1..=N {
        let first = i.to_string().chars().next().unwrap().to_digit(10).unwrap() as usize;
        let last = i.to_string().chars().last().unwrap().to_digit(10).unwrap() as usize;
        nums[first][last] += 1;
    }
    let mut ans = 0;
    for i in 0..=9 {
        for j in 0..=9 {
            ans += nums[i][j] * nums[j][i];
        }
    }
    println!("{}", ans);
}
