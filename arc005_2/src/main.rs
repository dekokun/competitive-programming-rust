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
    let mut add_x = 0;
    let mut add_y = 0;
    for c in W.chars() {
        match c {
            'R' => add_x = 1,
            'L' => add_x = -1,
            'U' => add_y = -1,
            'D' => add_y = 1,
            _ => panic!("おかしい"),
        }
    }
    let mut ans = "".to_string();
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
