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
    let n: u64 = read();
    let mut fives = vec![5];
    while fives[fives.len() - 1] <= n / 5 {
        fives.push(fives[fives.len() - 1] * 5);
    }
    for (i, v) in fives.into_iter().enumerate() {
        if (n - v) % 3 != 0 {
            continue;
        }
        let mut third_pow = 1;
        let mut three = 0;
        while third_pow <= n - v {
            // dbg!(v, n - v, third_pow);
            if third_pow == (n - v) {
                println!("{} {}", three, i + 1);
                return;
            }
            three += 1;
            if third_pow > std::u64::MAX / 3 {
                break;
            }
            third_pow *= 3;
        }
    }
    println!("-1");
}
