#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::BinaryHeap;

use proconio::input;

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    println!("{}", solve(a, b, c));
}

fn solve(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> usize {
    let mut a: BinaryHeap<usize> = a.into();
    let mut b: BinaryHeap<usize> = b.into();
    let mut c: BinaryHeap<usize> = c.into();
    let mut ans = 0;
    'outer: while let Some(c) = c.pop() {
        while let Some(b) = b.pop() {
            if b >= c {
                continue;
            }
            while let Some(a) = a.pop() {
                if a >= b {
                    continue;
                }
                ans += 1;
                continue 'outer;
            }
        }
    }
    ans
}
