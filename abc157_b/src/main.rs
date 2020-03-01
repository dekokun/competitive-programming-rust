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
    let mut bingo: Vec<Vec<usize>> = vec![];
    bingo.push(vec![read(), read(), read()]);
    bingo.push(vec![read(), read(), read()]);
    bingo.push(vec![read(), read(), read()]);
    let mut bingo_result: Vec<Vec<bool>> = vec![vec![false; 3]; 3];
    let N: usize = read();
    for _ in 0..N {
        let v: usize = read();
        for i in 0..3 {
            for j in 0..3 {
                if bingo[i][j] == v {
                    bingo_result[i][j] = true;
                }
            }
        }
    }
    if bingo_result[0].iter().all(|&x| x)
        || bingo_result[1].iter().all(|&x| x)
        || bingo_result[2].iter().all(|&x| x)
    {
        println!("Yes");
        return;
    }
    if column(0, &bingo_result).iter().all(|&x| x)
        || column(1, &bingo_result).iter().all(|&x| x)
        || column(2, &bingo_result).iter().all(|&x| x)
    {
        println!("Yes");
        return;
    }
    if diagonal1(&bingo_result).iter().all(|&x| x) || diagonal2(&bingo_result).iter().all(|&x| x) {
        println!("Yes");
        return;
    }
    println!("No");
}

fn column(column_num: usize, bingo_result: &Vec<Vec<bool>>) -> Vec<bool> {
    vec![
        bingo_result[0][column_num],
        bingo_result[1][column_num],
        bingo_result[2][column_num],
    ]
}

fn diagonal1(bingo_result: &Vec<Vec<bool>>) -> Vec<bool> {
    vec![bingo_result[0][0], bingo_result[1][1], bingo_result[2][2]]
}

fn diagonal2(bingo_result: &Vec<Vec<bool>>) -> Vec<bool> {
    vec![bingo_result[0][2], bingo_result[1][1], bingo_result[2][0]]
}
