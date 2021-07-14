#![allow(non_snake_case, unused_macros, dead_code)]

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
        q: usize,
        b: [usize; q],
    }
    println!(
        "{}",
        solve(a, b)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(mut a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    a.sort();
    debug!(a);
    let mut ans = vec![];
    for v in b {
        let ret = a.binary_search(&v);
        ans.push(match ret {
            Ok(_) => 0,
            Err(0) => a[0] - v,
            Err(i) if i == a.len() => v - a[a.len() - 1],
            Err(i) => (a[i] - v).min(v - a[i - 1]),
        });
    }
    ans
}
