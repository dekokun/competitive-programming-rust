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
    let K: usize = read();
    use std::collections::VecDeque;
    let mut queue: VecDeque<i64> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i);
    }
    for _ in 0..K - 1 {
        let v = queue.pop_front().unwrap();
        let rem = v % 10;
        match rem {
            0 => {
                queue.push_back(v * 10 + rem);
                queue.push_back(v * 10 + rem + 1)
            }
            9 => {
                queue.push_back(v * 10 + rem - 1);
                queue.push_back(v * 10 + rem);
            }
            _ => {
                queue.push_back(v * 10 + rem - 1);
                queue.push_back(v * 10 + rem);
                queue.push_back(v * 10 + rem + 1)
            }
        }
    }
    println!("{}", queue.pop_front().unwrap());
}
