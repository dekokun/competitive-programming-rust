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
    let N: usize = read();
    let S: String = read();
    if N % 2 == 0 {
        println!("-1");
        return;
    }
    // 半分の場所は必ずb (Nが1始まりなことに注意)
    if S.chars().nth(N / 2) != Some('b') {
        println!("-1");
        return;
    }
    let mut before = '_';
    for i in 0..N {
        let c = S.chars().nth(i).unwrap();
        match before {
            'a' => {
                if c != 'b' {
                    println!("-1");
                    return;
                }
            }
            'b' => {
                if c != 'c' {
                    println!("-1");
                    return;
                }
            }
            'c' => {
                if c != 'a' {
                    println!("-1");
                    return;
                }
            }
            _ => {}
        }
        before = c;
    }
    // ここまで来たならもうOKなので文字数から増えた数を計算
    println!("{}", (N - 1) / 2);
}
