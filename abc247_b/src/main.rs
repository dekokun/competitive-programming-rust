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
        st: [(String, String); n]
    }
    println!("{}", solve(st));
}

fn solve(st: Vec<(String, String)>) -> String {
    let mut map = HashMap::new();
    for (s, t) in &st {
        let entry = map.entry(s).or_insert(0);
        *entry += 1;
        // if s == t {
        //     continue;
        // }
        let entry = map.entry(t).or_insert(0);
        *entry += 1;
    }
    for (s, t) in &st {
        if map[s] >= 2 && map[t] >= 2 {
            return "No".into()
        }
    }
    "Yes".into()
}
