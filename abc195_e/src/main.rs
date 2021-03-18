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
        s: String,
        x: String,
    }
    println!(
        "{}",
        solve(
            n,
            s.chars().map(|c| c.to_digit(10).unwrap()).collect(),
            x.chars().collect()
        )
    );
}

fn solve(_n: usize, mut s: Vec<u32>, mut x: Vec<char>) -> String {
    s.reverse();
    x.reverse();
    let mut state = HashSet::new();
    state.insert(0);
    for i in 0..s.len() {
        let mut new_state = HashSet::new();
        for j in 0..7 {
            if x[i] == 'A' {
                if state.contains(&((j * 10 + s[i]) % 7)) && state.contains(&((j * 10) % 7)) {
                    new_state.insert(j);
                }
            } else {
                if state.contains(&((j * 10 + s[i]) % 7)) || state.contains(&((j * 10) % 7)) {
                    new_state.insert(j);
                }
            }
        }
        state = new_state;
        debug!(state);
    }
    if state.contains(&0) {
        "Takahashi"
    } else {
        "Aoki"
    }
    .into()
}
