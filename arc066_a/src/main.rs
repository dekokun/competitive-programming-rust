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
    let mut vec: Vec<usize> = vec![0; N];
    let mut zero_flag = false;
    for _ in 0..N {
        let v: usize = read();
        if v == 0 {
            zero_flag = true;
        }
        vec[v] += 1;
    }
    if zero_flag && vec[0] != 1 {
        println!("0");
        return;
    }
    let start = if zero_flag { 0 } else { 1 };
    for i in start..N {
        if i != 0 && (i - start) % 2 == 0 && vec[i] != 2 {
            println!("0");
            return;
        }
        if (i - start) % 2 == 1 && vec[i] != 0 {
            println!("0");
            return;
        }
    }
    let n = if zero_flag { N - 1 } else { N } / 2;
    let mut ans = 1;
    for _ in 0..n {
        ans *= 2;
        ans %= 10usize.pow(9) + 7;
    }
    println!("{}", ans);
}
