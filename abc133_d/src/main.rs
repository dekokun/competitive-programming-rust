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

/*
- x
  - a
- y
  - b
- z
  - c
a = (x + y) / 2
=> y = 2a - x
b = (y + z) / 2
=> z = 2b - y => z = 2b - 2a + x
c = (z + x) / 2
=> x = 2c - z => x = 2c - 2b + 2a - x
=> x = a - b + c
*/
fn main() {
    let N: usize = read();
    let mut A: Vec<usize> = (0..N).map(|_| read()).collect();
    let mut ans: Vec<usize> = vec![];
    let mut firstVal: i32 = 0;
    for (i, &v) in A.iter().enumerate() {
        if i % 2 == 0 {
            firstVal += v as i32;
        } else {
            firstVal -= v as i32;
        }
    }
    A.pop();
    ans.push(firstVal as usize);
    let mut now = firstVal as usize;
    for dam in A {
        let next = (dam - (now as usize / 2)) * 2;
        now = next;
        ans.push(next);
    }
    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
