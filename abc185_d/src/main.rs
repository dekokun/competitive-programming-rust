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
    let m: usize = read();
    if m == 0 {
        println!("1");
        return;
    }
    // true: 青, false: 白
    let mut boxes = vec![false;n];
    for _ in 0..m {
        let a: usize = read();
        boxes[a - 1] = true;
    }
    let mut min = n;
    let mut cons = 0;
    let mut conses = vec![];
    for &v in &boxes {
        if v {
            if cons == 0 {
                continue;
            }
            min = cons.min(min);
            conses.push(cons);
            cons = 0;
        } else {
            cons += 1;
        }
    }
    if cons != 0 {
        min = cons.min(min);
        conses.push(cons);
    }
    let k = min;
    let mut ans = 0;
    for v in conses {
        ans += ((v - 1) / k) + 1;
    }
    println!("{}", ans)
}
