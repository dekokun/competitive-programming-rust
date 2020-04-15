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
    let N: usize = read();
    let mut ans = 0;
    let mut last_a = 0;
    let mut first_b = 0;
    let mut loop_flag = true;
    for _ in 0..N {
        let s: String = read();
        let first_str = &s[0..1];
        let last_str = &s[s.len() - 1..s.len()];

        if first_str == "B" {
            first_b += 1;
            if last_str != "A" {
                loop_flag = false;
            }
        }
        if last_str == "A" {
            last_a += 1;
            if first_str != "B" {
                loop_flag = false;
            }
        }

        for s2 in s.chars().collect::<Vec<char>>().windows(2) {
            if s2 == &(vec!['A', 'B'][..]) {
                ans += 1;
            }
        }
    }
    ans += std::cmp::min(last_a, first_b);
    if first_b == 0 && last_a == 0 {
        loop_flag = false;
    }
    ans += if loop_flag { -1 } else { 0 };
    println!("{}", ans)
}
