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
    let n: i64 = read();
    let w: i64 = read();
    let mut imos = vec![0; 2 * 10_usize.pow(5) + 1];
    for _ in 0..n {
        let s: i64 = read();
        let t: i64 = read();
        let p: i64 = read();
        imos[s as usize] += p;
        imos[t as usize] += -p;
    }
    let mut now = 0;
    for v in imos {
        now += v;
        if now > w {
            println!("No");
            return
        }
    }
    println!("Yes");
}
