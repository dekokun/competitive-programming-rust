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
    let age: i32 = read();
    let price: i32 = read();
    let ret = if age <= 5 {
        0
    } else if age <= 12 {
        price / 2
    } else {
        price
    };
    println!("{}", ret);
}
