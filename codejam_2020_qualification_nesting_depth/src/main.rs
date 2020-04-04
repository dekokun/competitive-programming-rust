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
    let T: usize = read();
    for t in 1..T + 1 {
        let S: String = read();
        let mut ans: String = "".to_string();
        let mut before = 0;
        for c in S.chars() {
            let d: usize = c.to_digit(10).unwrap() as usize;
            if d > before {
                ans += &"(".repeat(d - before);
            }
            if before > d {
                ans += &")".repeat(before - d);
            }
            ans += &d.to_string();
            before = d;
        }
        if before != 0 {
            ans += &")".repeat(before);
        }

        println!("Case #{}: {}", t, ans);
    }
}
