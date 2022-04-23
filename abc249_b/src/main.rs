#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashSet;

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
        s: String,
    }
    println!("{}", solve(s));
}

fn solve(s: String) -> String {
    let mut komoji_exist = false;
    let mut omoji_exist = false;
    let mut differ = true;
    let mut sts = HashSet::new();
    for c in s.chars() {
        if c.is_uppercase() {
            omoji_exist = true;
        }
        if c.is_lowercase() {
            komoji_exist = true;
        }
        if sts.contains(&c) {
            differ = false;
        }
        sts.insert(c);
    }
    if omoji_exist && komoji_exist && differ {
        "Yes"
    } else {
        "No"
    }.into()
}
