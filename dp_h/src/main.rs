#![allow(non_snake_case)]
use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let MOD: i64 = 10_i64.pow(9) + 7;
    let h: usize = read();
    let w: usize = read();
    let mut map: Vec<Vec<Option<i64>>> = vec![ vec![None; w]; h ];
    for i in 0..h {
        let str: String = read();
        for (j, c) in str.chars().into_iter().enumerate() {
            map[i][j] = match c {
                '.' => Some(0),
                '#' => None,
                _ => panic!("strange char")
            };
        }
    }
    map[0][0] = Some(1);
    for i in 0..h {
        for j in 0..w {
            if map[i][j].is_none() || (i == 0 && j == 0) {
                continue;
            }

            map[i][j] = if (i == 0 || map[i - 1][j].is_none()) && (j == 0 || map[i][j - 1].is_none()) {
                Some(0)
            } else if i == 0 || map[i - 1][j].is_none() {
                map[i][j - 1]
            } else if j == 0 || map[i][j - 1].is_none() {
                map[i - 1][j]
            } else {
                Some((map[i - 1][j].unwrap() + map[i][j - 1].unwrap()) % MOD)
            };
        }
    }
    println!("{}", map[h - 1][w - 1].unwrap());
}
