use std::io::*;
use std::str::FromStr;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n: i32 = read();
    let k: i32 = read();
    let s: String = read();
    let mut score: i32 = n - 1;
    let mut before = s.chars().collect::<Vec<_>>()[0];
    for c in s.chars() {
        if before != c {
            score = std::cmp::max(0, score - 1);
        }
        before = c;
    }
    println!("{}", std::cmp::min(n - 1, score + k * 2));
}
