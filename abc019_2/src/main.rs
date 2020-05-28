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
    let s: String = read();
    let mut before = None;
    let mut seq = 1;
    let mut ans = vec![];
    for c in s.chars() {
        if before.is_none() {
            before = Some(c);
            continue;
        }
        if before.unwrap() == c {
            seq += 1;
        } else {
            ans.push((before.unwrap(), seq));
            seq = 1;
        }
        before = Some(c);
    }
    ans.push((before.unwrap(), seq));
    println!(
        "{}",
        ans.into_iter()
            .map(|(c, count)| c.to_string() + &count.to_string())
            .collect::<Vec<_>>()
            .join("")
    )
}

#[allow(dead_code)]
fn solve() {}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, true);
    }
}
