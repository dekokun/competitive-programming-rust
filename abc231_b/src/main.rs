#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashMap;

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    println!("{}", solve(n, s));
}

fn solve(n: usize, s: Vec<String>) -> String {
    let mut m = HashMap::new();
    for name in s {
        let entry = m.entry(name).or_insert(0);
        *entry += 1;
    }
    let mut max = 0;
    let mut ans = "".to_string();
    for (name, v) in m {
        if max <= v {
            max = v;
            ans = name;
        }
    }
    ans.into()
}
