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
    println!("{}", solve(read()));
}

#[allow(dead_code)]
fn solve(s: String) -> usize {
    let mut after = vec![];
    let mut before = s.chars().next().unwrap();
    for c in s.chars().skip(1) {
        if before == 'B' && c == 'C' {
            before = 'D';
        } else {
            after.push(before);
            before = c;
        }
    }
    after.push(before);
    let mut ans = 0;
    let mut tmp = 0;
    for c in after {
        match c {
            'A' => tmp += 1,
            'D' => {
                ans += tmp;
            }
            'C' | 'B' => {
                tmp = 0;
            }
            a => panic!("{}", a),
        }
    }
    ans
}
