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
    let n: usize = read();
    let mut vec: Vec<usize> = (0..n).map(|_| read()).collect();
    vec.sort();
    let mut vec2: Vec<bool> = vec![false; 10_usize.pow(6) + 2];
    for &v in &vec {
        vec2[v] = true;
    }
    let mut ans = vec![];
    // 一番小さな数が1個だけの場合はそう。
    // 二番目に小さな数は1番目の倍数じゃなかったらそう
    // ソートしてエラトステネスの篩でFAです。ただし次が同じ数かどうかは要注意
    for i in 0..vec.len() {
        let v = vec[i];
        if vec2[v] {
            if i == vec.len() - 1 || v != vec[i + 1] {
                ans.push(vec[i]);
            }
            let mut now = v;
            let org = now;
            while now <= 10_usize.pow(6) {
                vec2[now] = false;
                now += org;
            }
        }
    }
    println!("{}", ans.len());
}
