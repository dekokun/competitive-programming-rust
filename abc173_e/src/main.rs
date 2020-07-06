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
    let N: usize = read();
    let K: usize = read();
    let mut vec: Vec<i32> = (0..N).map(|_| read()).collect();
    let MOD = 10_i64.pow(9) + 7;
    vec.sort();
    if vec.iter().all(|&x| x < 0) && K % 2 == 1 {
        vec.reverse();
        let mut tmp: i64 = 1;
        for &i in &vec[0..K] {
            tmp *= i as i64;
            tmp %= MOD;
        }
        println!("{}", tmp + MOD);
        return;
    }
}
