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
    let N: String = read();
    let chars = N.chars();
    let mut carry = 0;
    let mut ans = 0;
    let mut peekable = chars.rev().peekable();
    while let Some(c) = peekable.next() {
        let now = c.to_digit(10).unwrap() + carry;
        if now == 10 {
            carry = 1;
        } else if now == 5 {
            match peekable.peek() {
                None => {
                    carry = 0;
                    ans += now;
                }
                Some(c2) => {
                    let c2 = c2.to_digit(10).unwrap();
                    if c2 >= 5 {
                        carry = 1;
                        ans += 5;
                    } else {
                        carry = 0;
                        ans += 5;
                    }
                }
            }
        } else if now >= 6 {
            ans += 10 - now;
            carry = 1;
        } else {
            carry = 0;
            ans += now;
        }
    }
    if carry == 1 {
        ans += 1;
    }
    println!("{}", ans);
}
