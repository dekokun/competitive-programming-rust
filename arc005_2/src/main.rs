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
    let x: isize = read::<isize>() - 1;
    let y: isize = read::<isize>() - 1;
    let W: String = read();
    let mut card: Vec<Vec<usize>> = vec![vec![]; 9];
    for row in &mut card {
        let raw_row: String = read();
        for c in raw_row.chars() {
            let n = c.to_digit(10).unwrap() as usize;
            row.push(n);
        }
    }
    let (add_x, add_y): (isize, isize) = match W.as_str() {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        "RU" => (1, -1),
        "RD" => (1, 1),
        "LU" => (-1, -1),
        "LD" => (-1, 1),
        _ => panic!("unimplemented"),
    };
    let mut ans = "".to_string();
    let mut add_x = add_x;
    let mut add_y = add_y;
    let mut x = x;
    let mut y = y;
    for _ in 0..4 {
        ans += &card[y as usize][x as usize].to_string();
        x += add_x;
        y += add_y;
        if x == 0 || x == 8 {
            add_x = -add_x;
        }
        if y == 0 || y == 8 {
            add_y = -add_y;
        }
    }
    println!("{}", ans);
}