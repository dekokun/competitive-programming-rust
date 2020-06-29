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
    let mut ans = 0;
    for index in 0..n {
        let i = index + 1;
        if vec[index] == i {
            ans += 1;
            if index == n - 1 {
                vec.swap(n - 1, n - 2);
                continue;
            }
            vec.swap(index, index + 1);
        }
    }
    println!("{}", ans)
}
