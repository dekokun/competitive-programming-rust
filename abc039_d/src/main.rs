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
    let mut map: Vec<Vec<char>> = vec![];
    for _ in 0..H {
        let row: Vec<char> = read::<String>().chars().collect();
        map.push(row);
    }
    let mut before_map: Vec<Vec<char>> = vec![vec!['#'; W]; H];
    for i in 0..H {
        for j in 0..W {
            for addy in &[-1, 0, 1] {
                for addx in &[-1, 0, 1] {
                    if i as i32 + addy < 0
                        || j as i32 + addx < 0
                        || H as i32 <= i as i32 + addy
                        || W as i32 <= j as i32 + addx
                    {
                        continue;
                    }
                    let nexty = (i as i32 + addy) as usize;
                    let nextx = (j as i32 + addx) as usize;
                    if map[nexty][nextx] == '.' {
                        before_map[i][j] = '.';
                    }
                }
            }
        }
    }
    let mut next_before_map: Vec<Vec<char>> = vec![vec!['.'; W]; H];
    for i in 0..H {
        for j in 0..W {
            for addy in &[-1, 0, 1] {
                for addx in &[-1, 0, 1] {
                    if i as i32 + addy < 0
                        || j as i32 + addx < 0
                        || H as i32 <= i as i32 + addy
                        || W as i32 <= j as i32 + addx
                    {
                        continue;
                    }
                    let nexty = (i as i32 + addy) as usize;
                    let nextx = (j as i32 + addx) as usize;
                    if before_map[nexty][nextx] == '#' {
                        next_before_map[i][j] = '#';
                    }
                }
            }
        }
    }
    if map == next_before_map {
        println!("possible");
        for row in before_map {
            println!(
                "{}",
                row.into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            )
        }
    } else {
        println!("impossible");
    }
}
