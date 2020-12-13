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
    let n: u64 = read();
    // n-1 C 11 を求める
    let mut ans = 1;
    use std::collections::HashSet;
    let mut set: HashSet<u64> = (1..=11).collect();

    for i in 0..11 {
        ans *= n - 1 - i;
        for v in set.clone() {
            if ans % v == 0 {
                ans /= v;
                set.remove(&v);
            }
        }
    }

    println!("{}", ans)
}
