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
    let n: usize = read();
    let vec: Vec<i32> = (0..n).map(|_| read()).collect();
    let diffs: Vec<_> = vec.windows(2).map(|a| a[1] - a[0]).collect();
    let mut before = None;
    let mut ans = 0;
    // dbg!(&diffs);
    for v in diffs {
        if v < 0 {
            match before {
                None => before = Some(v),
                Some(a) if a <= 0 => before = Some(v),
                Some(a) if a > 0 => {
                    before = None;
                    ans += 1;
                }
                Some(_) => unreachable!(),
            }
        } else if v > 0 {
            match before {
                None => before = Some(v),
                Some(a) if a >= 0 => before = Some(v),
                Some(a) if a < 0 => {
                    before = None;
                    ans += 1;
                }
                Some(_) => unreachable!(),
            }
        } else if v == 0 {
            match before {
                None => before = Some(v),
                Some(_) => {}
            }
        }
    }
    println!("{}", ans + 1)
}
