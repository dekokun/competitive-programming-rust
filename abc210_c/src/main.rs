#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

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
        k: usize,
        c: [usize; n],
    }
    println!("{}", solve(n, k, c));
}

fn solve(_n: usize, k: usize, c: Vec<usize>) -> usize {
    let mut ans = 0;
    let mut map = HashMap::new();
    for (i, &color) in c.iter().enumerate() {
        let entry = map.entry(color).or_insert(0);
        *entry += 1;
        if i < k {
            ans = ans.max(map.len());
            continue;
        }
        match map.get(&c[i - k]) {
            Some(1) => {
                map.remove(&c[i - k]);
            }
            Some(0) => panic!(),
            Some(&a) => {
                map.insert(c[i - k], a - 1);
            }
            _ => panic!(),
        }
        debug!(map, ans);
        ans = ans.max(map.len())
    }
    ans
}
