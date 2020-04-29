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
    let mut vec: Vec<usize> = vec![];
    for _ in 0..N {
        vec.push(read());
    }
    let mut ans_rev = vec![];
    for _ in 0..N {
        let mut last = None;
        for (i, &v) in vec.iter().enumerate() {
            let i = i + 1;
            if i == v {
                last = Some(i);
            }
        }
        match last {
            None => {
                println!("-1");
                return;
            }
            Some(a) => {
                ans_rev.push(a);
                vec.remove(a - 1);
            }
        }
    }
    ans_rev.reverse();
    for i in ans_rev {
        println!("{}", i);
    }
}
