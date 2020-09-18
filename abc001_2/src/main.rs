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
    let m: f64 = read();
    let km = m / 1000.0;
    let ans = if km < 0.1 {
        "00".to_owned()
    } else if km <= 5.0 {
        let ans = (km * 10.0).to_string();
        if ans.len() == 1 {
            "0".to_owned() + &ans
        } else {
            ans
        }
    } else if km <= 30.0 {
        (km + 50.0).to_string()
    } else if km <= 70.0 {
        (((km - 30.0) / 5.0) + 80.0).to_string()
    } else {
        "89".to_owned()
    };
    println!("{}", ans)
}
