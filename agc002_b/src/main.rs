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
    let n: usize = read();
    let m: usize = read();
    let mut balls: Vec<(bool, usize)> = vec![(false, 1); n];
    balls[0] = (true, 1);
    for _ in 0..m {
        let x: usize = read::<usize>() - 1;
        let y: usize = read::<usize>() - 1;
        let before = balls[x];
        let after = balls[y];
        let (next_1, next_2) = match (before, after) {
            ((_, 0), _) => panic!("a"),
            ((true, 1), (_, b)) => ((false, 0), (true, b + 1)),
            ((true, a), (_, b)) => ((true, a - 1), (true, b + 1)),
            ((false, a), (b, c)) => ((false, a - 1), (b, c + 1)),
        };
        balls[x] = next_1;
        balls[y] = next_2;
    }
    println!("{}", balls.into_iter().filter(|(b, _)| *b).count())
}
