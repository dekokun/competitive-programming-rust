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
    // use std::cmp::Reverse;
    // use std::collections::BinaryHeap;
    // let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    let mut vec: Vec<usize> = vec![];
    for _ in 0..n {
        let v = read();
        vec.push(v);
    }
    use std::collections::HashSet;
    while vec.len() > 1 {
        let mut min = 10_usize.pow(9);
        for &v in &vec {
            min = std::cmp::min(min, v);
        }
        let mut set = HashSet::new();
        for &v in &vec {
            let rem = v % min;
            if rem == 0 {
                set.insert(min);
                continue;
            }
            set.insert(rem);
        }
        vec = set.into_iter().collect();
    }
    println!("{}", vec[0])
}
