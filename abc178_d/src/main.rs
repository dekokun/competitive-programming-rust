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
use std::collections::HashMap;

fn main() {
    let S: u64 = read();
    let mut memo: HashMap<u64, u64> = HashMap::new();
    let MOD = 10_u64.pow(9) + 7;
    println!("{}", rec_memo(S, MOD, &mut memo));
}
fn rec_memo(n: u64, MOD: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 => 0,
        1 => 0,
        2 => 0,
        3 => 1,
        4 => 1,
        5 => 1,
        6 => 2,
        _ => match memo.get(&n) {
            None => {
                let mut new_num = 1; // 1はそれ自身
                for i in 3..=n - 3 {
                    new_num += rec_memo(i, MOD, memo);
                    new_num %= MOD;
                }
                memo.insert(n, new_num);
                new_num
            }
            Some(&a) => a,
        },
    }
}
