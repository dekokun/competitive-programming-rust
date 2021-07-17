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
        v: [usize; 3],
    }
    println!("{}", solve(n, v));
}

fn solve(n: usize, mut v: Vec<usize>) -> usize {
    v.sort();
    v.reverse();
    debug!(v);
    let mut ans = 10000;
    for i in 0..10000 {
        let sum = i * v[0];
        if sum > n {
            break;
        }
        for j in 0..10000 - i {
            let sum = i * v[0] + j * v[1];
            if sum > n {
                break;
            }
            if (n - sum) % v[2] == 0 {
                ans = ans.min(i + j + (n - sum) / v[2]);
            }
        }
    }
    debug!(n);
    ans
}
