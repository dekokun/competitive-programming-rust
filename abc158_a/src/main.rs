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

    let S: String = read();
    let v: Vec<_> = S.chars().collect();
    if (v[0] == 'B' && v[1] == 'B' && v[2] == 'B') ||(v[0] == 'A' && v[1] == 'A' && v[2] == 'A') {
        println!("No");
    } else {
        println!("Yes");
    }
}
