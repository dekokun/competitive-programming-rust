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
    use std::f64::consts::PI;
    let A: f64 = read();
    let B: f64 = read();
    let H: f64 = read();
    let M: f64 = read();
    // 余弦定理を使う
    let hour_rad = PI * 2.0 * ((H / 12.0) + (M / 60.0) / 12.0);
    let minute_rad = PI * 2.0 * M / 60.0;
    let diff_rad = hour_rad - minute_rad;
    let ans = (A.powi(2) + B.powi(2) - 2.0 * A * B * diff_rad.cos()).sqrt();
    println!("{}", ans)
}
