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
    let N: u8 = read();

    let ans = dfs("".to_string(), N as usize);
    for s in ans {
        println!("{}", s);
    }
}

fn dfs(now: String, max: usize) -> Vec<String> {
    let mut ans = vec![];
    if now.len() == max {
        ans.push(now);
        return ans;
    }
    let mut a = dfs(now.clone() + "a", max);
    let mut b = dfs(now.clone() + "b", max);
    let mut c = dfs(now + "c", max);
    ans.append(&mut a);
    ans.append(&mut b);
    ans.append(&mut c);
    ans
}