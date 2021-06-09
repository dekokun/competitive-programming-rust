#![allow(non_snake_case, unused_macros, dead_code)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}
use std::collections::VecDeque;
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
    let q: usize = read();
    let mut deq = VecDeque::new();

    for _ in 0..q {
        let t: usize = read();
        let x: usize = read();
        match t {
            1 => deq.push_front(x),
            2 => deq.push_back(x),
            3 => println!("{}", deq.get(x - 1).unwrap()),
            _ => panic!(),
        }
    }
}
