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
    for _ in 0..1 {
        map.push(vec!['#'; W + 2]);
    }
    for _ in 0..H {
        let mut row = vec!['#'; 1];
        let mut string: Vec<char> = read::<String>().chars().collect();
        row.append(&mut string);
        row.append(&mut vec!['#'; 1]);
        map.push(row);
    }
    for _ in 0..1 {
        map.push(vec!['#'; W + 2]);
    }
    for v in &map {
        let row = v
            .iter()
            .map(|&c| c.to_string())
            .collect::<Vec<_>>()
            .join("");
        eprintln!("{}", row);
    }
    for i in 0..H {
        let mut continuous = 0;
        for j in 0..W {
            match map[i][j] {
                '#' => {
                    continuous += 1;
                }
                '.' => {
                    if 0 < continuous && continuous < 3 {
                        println!("impossible");
                        return;
                    }
                    continuous = 0;
                }
                _ => unimplemented!(),
            }
        }
    }
    for j in 0..W {
        let mut continuous = 0;
        for i in 0..H {
            match map[i][j] {
                '#' => {
                    continuous += 1;
                }
                '.' => {
                    if 0 < continuous && continuous < 3 {
                        println!("impossible");
                        return;
                    }
                    continuous = 0;
                }
                _ => unimplemented!(),
            }
        }
    }
}
