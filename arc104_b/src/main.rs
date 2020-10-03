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
    let _n: usize = read();
    let s: String = read();
    // A, T, C, G
    let mut counts: Vec<(usize, usize, usize, usize)> = vec![(0, 0, 0, 0)];
    let mut before = (0, 0, 0, 0);
    for c in s.chars() {
        before = if c == 'A' {
            (before.0 + 1, before.1, before.2, before.3)
        } else if c == 'T' {
            (before.0, before.1 + 1, before.2, before.3)
        } else if c == 'C' {
            (before.0, before.1, before.2 + 1, before.3)
        } else if c == 'G' {
            (before.0, before.1, before.2, before.3 + 1)
        } else {
            unreachable!()
        };
        counts.push(before);
    }
    let mut ans = 0;
    for i in 0..counts.len() {
        for j in i + 1..counts.len() {
            let count1 = counts[i];
            let count2 = counts[j];
            let a = count2.0 - count1.0;
            let t = count2.1 - count1.1;
            let g = count2.2 - count1.2;
            let c = count2.3 - count1.3;
            if a == t && g == c {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
