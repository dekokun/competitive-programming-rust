#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashSet;

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
        ss: [String; n],
    }
    println!(
        "{}",
        solve(ss)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(ss: Vec<String>) -> Vec<usize> {
    let mut m: HashSet<String> = HashSet::new();
    let mut ans = vec![];
    for (i, name) in ss.into_iter().enumerate() {
        if m.contains(&name) {
            continue;
        }
        ans.push(i + 1);
        m.insert(name);
    }
    ans
}
