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
    let S: String = read();
    // xを除いて回文じゃなかったら-1
    // 回文だったらある文字とある文字の間にxが何個あるかを回文と比較
    let non_x = S.replace("x", "");
    let non_x_rev: String = non_x.chars().rev().collect();
    if non_x != non_x_rev {
        println!("-1");
        return;
    }
    // 計算の都合上xを前後に足しておく
    let S = "x".to_string() + &S + "x";
    let chars: Vec<_> = S.chars().collect();
    let mut i = 0;
    let count = S.chars().count();
    let mut x_counts = vec![];
    while i < count {
        let mut skip_count = 0;
        while i < count && chars[i] == 'x' {
            skip_count += 1;
            i += 1;
        }
        x_counts.push(skip_count);
        i += 1;
    }
    let x_counts_rev: Vec<_> = x_counts.iter().rev().collect();
    let mut ans = 0;
    for (i, v) in x_counts.iter().enumerate() {
        ans += std::cmp::max(v, x_counts_rev[i]) - std::cmp::min(v, x_counts_rev[i]);
    }
    println!("{}", ans / 2);
}
