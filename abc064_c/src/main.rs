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
    let N: usize = read();
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut big_user_num = 0;
    for _ in 0..N {
        let v: usize = read();
        let quot = v / 400;
        if quot <= 7 {
            set.insert(quot);
        } else {
            big_user_num += 1;
        }
    }
    println!(
        "{} {}",
        std::cmp::max(set.len(), 1),
        set.len() + big_user_num
    );
}
