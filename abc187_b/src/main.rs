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
    let v: Vec<(i32, i32)> = (0..n).map(|_| (read(), read())).collect();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = v[i];
            let (x2, y2) = v[j];
            let tilt = (y1 - y2) as f64 / (x1 - x2) as f64;
            if -1.0 <= tilt && tilt <= 1.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
