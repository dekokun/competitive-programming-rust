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
    let mut K: usize = read();
    let mut ans: String = "".to_owned();
    for c in s.chars() {
        let point = c as u8 - b'a';
        if c == 'a' {
            ans.push('a');
            continue;
        }
        if 26 - point as usize <= K {
            K -= 26 - point as usize;
            ans.push('a');
        } else {
            ans.push(c);
        }
    }
    if K != 0 {
        let last = ans.pop().unwrap();
        ans.push(((((last as usize - b'a' as usize) + K) % 26) as u8 + b'a') as char)
    }
    println!("{}", ans)
}
