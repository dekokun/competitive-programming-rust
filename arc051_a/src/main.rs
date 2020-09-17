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
    let (x1, y1): (i32, i32) = (read(), read());
    let r: i32 = read();
    let (x2, y2): (i32, i32) = (read(), read());
    let (x3, y3): (i32, i32) = (read(), read());
    // 円が全部4点の中かどうか(x1, y1にrを足し引きしたものがx2, y2, x3, y3の中にあるかどうか)
    if x1 + r <= x3 && x1 - r >= x2 && y1 + r <= y3 && y1 - r >= y2 {
        println!("NO");
    } else {
        println!("YES");
    }
    // 4点が全部円の中かどうか
    for (x, y) in &[(x2, y2), (x3, y3), (x2, y3), (x3, y2)] {
        if r.pow(2) > (x - x1).pow(2) + (y - y1).pow(2) {
            continue;
        }
        println!("YES");
        return
    }
    println!("NO");
}
