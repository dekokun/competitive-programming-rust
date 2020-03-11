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
    let _: usize = read();
    let S: String = read();
    // "(" は左から、")"は右から埋めることで辞書順最小を実現
    let mut stack: Vec<char> = vec![];
    for c in S.chars() {
        if stack.is_empty() {
            stack.push(c);
            continue;
        }
        let last = stack.pop().unwrap();
        if last == c || (last == ')' && c == '(') {
            stack.push(last);
            stack.push(c);
        }
    }
    let left_count = stack
        .iter()
        .fold(0, |acc, &x| if x == ')' { acc + 1 } else { acc });
    let right_count = stack.len() - left_count;
    println!(
        "{}",
        (0..left_count).map(|_| '(').collect::<String>()
            + &S
            + &(0..right_count).map(|_| ')').collect::<String>()
    );
}
