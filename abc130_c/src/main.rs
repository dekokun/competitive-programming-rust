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
    let w: f64 = read();
    let h: f64 = read();
    let x: f64 = read();
    let y: f64 = read();
    let multi = if w / 2.0 == x && h / 2.0 == y {
        1
    } else {
        0
    };
    println!("{} {}", w * h / 2.0, multi);
}
