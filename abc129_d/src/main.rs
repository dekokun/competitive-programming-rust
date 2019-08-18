use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let h: i32 = read();
    let w: i32 = read();
    let mut vec = vec![];
    for i in 0..h {
        let str: String = read();
        vec.push(((*str).clone().chars().clone()).clone());
    }
}
