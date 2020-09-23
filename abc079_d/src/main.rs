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
    let mut c: Vec<Vec<i32>> = vec![vec![-1; 10]; 10];
    for i in 0..=9 {
        for j in 0..=9 {
            c[i][j] = read();
        }
    }
    for k in 0..=9 {
        for i in 0..=9 {
            for j in 0..=9 {
                if c[i][j] > c[i][k] + c[k][j] {
                    c[i][j] = c[i][k] + c[k][j];
                }
            }
        }
    }
    let mut ans = 0;
    for _ in 0..H {
        for _ in 0..W {
            let a: i32 = read();
            if a == -1 {
                continue;
            }
            let a = a as usize;
            ans += c[a][1];
        }
    }
    println!("{}", ans)
}
