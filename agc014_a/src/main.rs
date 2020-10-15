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
    let (a, b, c): (usize, usize, usize) = (read(), read(), read());
    let mut vec = vec![a, b, c];
    vec.sort();
    let mut ans = 0;
    use std::collections::HashSet;
    let mut set: HashSet<Vec<usize>> = HashSet::new();
    set.insert(vec.clone());
    while vec[0] % 2 == 0 && vec[1] % 2 == 0 && vec[2] % 2 == 0 {
        ans += 1;
        let mut after = vec![0; 3];
        after[0] = (vec[1] + vec[2]) / 2;
        after[1] = (vec[0] + vec[2]) / 2;
        after[2] = (vec[0] + vec[1]) / 2;
        after.sort();
        if set.contains(&after) {
            println!("-1");
            return;
        }
        set.insert(after.clone());
        vec = after;
    }
    println!("{}", ans);
}
