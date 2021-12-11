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
        q: usize,
        a: [usize; n],
        x: [usize; q]
    }
    println!(
        "{}",
        solve(a, x)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(mut a: Vec<usize>, x: Vec<usize>) -> Vec<usize> {
    a.sort();
    let mut ans = vec![];
    for i in x {
        let res = a.binary_search(&i);
        match res {
            Ok(c) => {
                let mut left = c;
                while left != 0 && a[left - 1] == a[left] {
                    left -= 1;
                }
                ans.push(a.len() - left);
            }
            Err(c) => ans.push(a.len() - c),
        }
    }
    ans
}
