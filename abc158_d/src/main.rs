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
    let Q: usize = read();
    let mut is_reverse = false;
    let mut before_string = "".to_string();
    let mut after_string = "".to_string();
    for _ in 0..Q {
        let T: usize = read();
        if T == 1 {
            is_reverse = !is_reverse;
        } else {
            let F: i32 = read();
            // 1だったら2に、2だったら1に
            let F = if is_reverse { -(F - 2) + 1 } else { F };
            let C: String = read();
            let C = C.chars().next().unwrap();
            if F == 1 {
                before_string.push(C);
            } else {
                after_string.push(C);
            }
        }
    }
    before_string = before_string.chars().rev().collect();
    let msg = before_string + &S + &after_string;
    let msg = if is_reverse {
        msg.chars().rev().collect()
    } else {
        msg
    };
    println!("{}", msg);
}
