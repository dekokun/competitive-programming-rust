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
    let (sx, sy, tx, ty): (isize, isize, isize, isize) = (read(), read(), read(), read());
    let tx = (tx - sx) as usize;
    let ty = (ty - sy) as usize;
    let mut ans = "".to_owned();
    // 0, 0の右からtx, tyの下
    ans += &"R".to_owned().repeat(tx);
    ans += &"U".to_owned().repeat(ty);
    // tx, tyの右から0, 0の下
    ans += "R";
    ans += &"D".to_owned().repeat(ty + 1);
    ans += &"L".to_owned().repeat(tx + 1);
    ans += "U";
    // 0, 0の上からtx, tyの左
    ans += &"U".to_owned().repeat(ty);
    ans += &"R".to_owned().repeat(tx);
    // tx, tyの上から0, 0の左
    ans += "U";
    ans += &"L".to_owned().repeat(tx + 1);
    ans += &"D".to_owned().repeat(ty + 1);
    ans += "R";

    println!("{}", ans);
}
