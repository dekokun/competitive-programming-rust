#![allow(non_snake_case, unused_macros, dead_code)]

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
    }
    println!(
        "{}",
        solve(n)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solve(n: usize) -> Vec<usize> {
    let mut ans = vec![];
    for i in 1..=n {
        let mut tmp = ans.clone();
        ans.push(i);
        ans.append(&mut tmp);
    }
    ans
}
