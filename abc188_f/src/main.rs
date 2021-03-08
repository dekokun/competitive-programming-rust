#![allow(non_snake_case)]

use std::{collections::HashSet, str::FromStr};
use std::{
    collections::VecDeque,
    io::{stdin, Read},
};
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
    println!("{}", solve(read(), read()));
}

fn solve(x: u64, y: u64) -> u64 {
    // y から x に変換する。
    // +1, -1, /2
    if x >= y {
        return x - y;
    }
    let mut queue = VecDeque::new();
    queue.push_front((y, 0));
    let mut dp: HashSet<u64> = HashSet::new();
    dp.insert(y);
    let mut ans = y;
    while let Some((v, count)) = queue.pop_back() {
        if v <= x {
            ans = ans.min(count + x - v);
            continue;
        }
        ans = ans.min(count + v - x);
        if v % 2 == 1 {
            for &v in &[v + 1, v - 1] {
                if !dp.contains(&v) {
                    queue.push_front((v, count + 1));
                    dp.insert(v);
                }
            }
        } else {
            if !dp.contains(&(v / 2)) {
                queue.push_front((v / 2, count + 1));
                dp.insert(v / 2);
            }
        }
    }
    ans
}
