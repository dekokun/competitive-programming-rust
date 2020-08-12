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
    let N: u64 = read();
    let K: u64 = read();
    use std::collections::VecDeque;
    let mut sub: VecDeque<u64> = VecDeque::new();
    let mut now: u64 = 1;
    let mut ans = 0;
    for _ in 0..N {
        let s: u64 = read();
        if s == 0 {
            println!("{}", N);
            return;
        }
        if now * s <= K {
            sub.push_back(s);
            ans = std::cmp::max(ans, sub.len());
            now *= s;
            continue;
        }
        // 1は最大限連続してほしい
        while now * s > K && now != 1 {
            let head = sub.pop_front().unwrap();
            now /= head;
        }
        // sがKより大きい時だけここに来る
        if now * s > K {
            now = 1;
            sub = VecDeque::new();
            continue;
        }
        sub.push_back(s);
        ans = std::cmp::max(ans, sub.len());
        now *= s;
    }
    println!("{}", ans);
}
