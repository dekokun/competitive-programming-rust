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
    let (x, y, a, b): (u64, u64, u64, u64) = (read(), read(), read(), read());
    let mut exp = 0;
    let mut str = x;
    loop {
        if str <= (str + b) / a {
            str *= a;
            if str >= y {
                println!("{}", exp);
                return;
            }
            exp += 1;
        } else {
            println!("{}", exp + (y - str - 1) / b);
            return;
        };
    }
}
