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
    let (x1, y1, x2, y2): (i32, i32, i32, i32) = (read(), read(), read(), read());
    // 何らかで(0, 0)を頂点にすると、もう一つとして(x, y)があったら(x -y, x + y), (-y, x)
    let (x, y) = (x2 - x1, y2 - y1);
    println!("{} {} {} {}", x - y + x1, x + y + y1, -y + x1, x + y1);
}
