#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

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
    for t in 1..=T {
        let m: usize = read();
        println!(
            "Case #{}: {}",
            t,
            solve((0..m).map(|_| (read(), read())).collect())
        );
    }
}

use std::collections::HashMap;
fn solve(vec: Vec<(u64, u64)>) -> u64 {
    let mut map = HashMap::new();
    let mut sum = 0;
    for (prime, count) in vec {
        sum += prime * count;
        map.insert(prime, count);
    }
    dfs(sum, 1, map, 600)
}

fn dfs(sum: u64, multi: u64, mut remain: HashMap<u64, u64>, before: u64) -> u64 {
    if sum == multi {
        return sum;
    }
    if sum < multi {
        return 0;
    }
    let rem2 = remain.clone();
    let mut ans = None;
    for (&prime, &count) in &rem2 {
        if prime >= before {
            remain.remove(&prime);
            continue;
        }
        remain.remove(&before);
        for i in 0..=count {
            if sum - prime * i < multi * prime.pow(i as u32) {
                break;
            }
            let new = dfs(
                sum - prime * i,
                multi * prime.pow(i as u32),
                remain.clone(),
                prime,
            );
            if ans.is_none() {
                ans = Some(new);
            } else {
                ans = Some(ans.unwrap().max(new));
            }
        }
    }
    if ans.is_none() {
        0
    } else {
        ans.unwrap()
    }
}
