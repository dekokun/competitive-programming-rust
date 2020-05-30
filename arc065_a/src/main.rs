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
    let mut vec: Vec<_> = S.chars().collect();
    let mut now_string = "".to_string();
    while !vec.is_empty() {
        let c = vec.pop().unwrap();
        now_string = now_string + &c.to_string();
        match now_string.as_str() {
            "maerd" | "remaerd" | "esare" | "resare" => {
                now_string = "".to_string();
            }
            _ => {
                if now_string.len() >= 7 {
                    println!("NO");
                    return;
                }
            }
        }
    }
    if now_string.len() != 0 {
        println!("NO");
    } else {
        println!("YES");
    }
}
