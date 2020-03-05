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
    let _: usize = read();
    let S: String = read();
    let mut e_count = 0;
    for c in S.chars() {
        if c == 'E' {
            e_count += 1;
        }
    }
    let mut ans = std::usize::MAX;
    let mut now_may_change_count = e_count;
    let mut before_char = 'X';
    for c in S.chars() {
        if c == 'E' {
            now_may_change_count -= 1;
        }
        if before_char == 'W' {
            now_may_change_count += 1;
        }
        before_char = c;
        ans = std::cmp::min(ans, now_may_change_count);
    }
    println!("{}", ans);
}
