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
        a: [usize; n]
    }
    println!("{}", solve(a));
}

fn solve(a: Vec<usize>) -> usize {
    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<Vec<(usize, usize)>>();
    a.sort();
    a.reverse();
    debug!(a);
    a[1].1 + 1
}
