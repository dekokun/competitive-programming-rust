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
    let a: i8 = read();
    let b: i8 = read();
    let mut socket_num = 1;
    let mut power_stripe_num = 0;
    while socket_num < b {
        socket_num += a - 1;
        power_stripe_num += 1;
    }
    println!("{}", power_stripe_num);
}
