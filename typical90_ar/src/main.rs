#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::VecDeque;

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
        q: usize,
        a: [usize; n],
        txy: [(usize, usize, usize); q]
    }
    println!(
        "{}",
        solve(a, txy)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(a: Vec<usize>, txy: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut a: VecDeque<usize> = a.into();
    debug!(a);
    let mut ans = vec![];
    for (t, x, y) in txy {
        match t {
            1 => {
                let tmp = a[x - 1];
                a[x - 1] = a[y - 1];
                a[y - 1] = tmp;
            }
            2 => {
                let last = a.pop_back().unwrap();
                a.push_front(last);
            }
            3 => {
                ans.push(a[x - 1]);
            }
            _ => panic!(),
        }
    }
    ans
}
