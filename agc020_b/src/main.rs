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
    let K: usize = read();
    let mut A: Vec<usize> = (0..K).map(|_| read()).collect();
    A.reverse();
    if *A.first().unwrap() > 2 {
        println!("{}", -1);
        return;
    }
    let mut min = 2;
    let mut max = 2;
    let mut a_max = A[0];
    for &a in &A[1..] {
        if a >= a_max * 2 {
            println!("{}", -1);
            return;
        }
        if a >= a_max {
            max = std::cmp::max(max, a * 2 - 1);
            a_max = a;
            min = std::cmp::max(a, min);
        }
        if a_max % a == 0 {
            continue;
        } else {
            // max は、aによってmax以下になるようになるまで増える
            if max % a != 0 {
                max += a - (max % a) - 1;
            }
            // minも増える
            if min % a != 0 {
                min += a - (min % a);
            }
        }
    }
    println!("{} {}", min, max)
}
