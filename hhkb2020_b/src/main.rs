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
    let (h, w): (usize, usize) = (read(), read());
    let mut v: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        let s: String = read();
        v.push(s.chars().collect());
    }
    let mut ans = 0;
    for line in v.clone() {
        for a in line.windows(2) {
            if a[0] == a[1] && a[0] == '.' {
                ans += 1;
            }
        }
    }
    for i in 0..w {
        for j in 0..h {
            if j == 0 {
                continue;
            }
            if v[j - 1][i] == v[j][i] && v[j][i] == '.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
