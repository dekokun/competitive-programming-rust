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
    let s: String = read();
    let t: String = read();
    let s: Vec<_> = s.chars().collect();
    let t: Vec<_> = t.chars().collect();
    let mut ans = 0;
    for (i, c) in s.iter().enumerate() {
        if *c == t[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
