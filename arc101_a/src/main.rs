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
    let mut N: usize = read();
    let mut K: usize = read();
    let mut minuses = vec![];
    let mut pluses = vec![];
    for _ in 0..N {
        let val: i32 = read();
        match val.cmp(&0) {
            std::cmp::Ordering::Equal => {
                N -= 1;
                K -= 1;
            }
            std::cmp::Ordering::Less => {
                minuses.push(-val as u64);
            }
            std::cmp::Ordering::Greater => {
                pluses.push(val as u64);
            }
        }
    }
    if K == 0 {
        println!("0");
        return;
    }
    minuses.reverse();
    let mut ans = std::u64::MAX;
    for (one, another) in &[(&minuses, &pluses), (&pluses, &minuses)] {
        for (i, &v) in one.iter().enumerate() {
            if K == i + 1 {
                ans = std::cmp::min(ans, v);
                break;
            }
            if K - i - 1 > another.len() {
                continue;
            }
            // dbg!(K, i, v, K - i - 2, another[K - i - 2]);
            ans = std::cmp::min(ans, v * 2 + another[K - i - 2]);
        }
    }
    println!("{}", ans);
}
