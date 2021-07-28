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
        b: [usize; n],
        c: [usize; n],
    }
    println!("{}", solve(a, b, c));
}

fn solve(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> u64 {
    let mut ma = vec![0; 46];
    for v in a {
        ma[v % 46] += 1;
    }
    let mut mb = vec![0; 46];
    for v in b {
        mb[v % 46] += 1;
    }
    let mut mc = vec![0; 46];
    for v in c {
        mc[v % 46] += 1;
    }
    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += ma[i] * mb[j] * mc[k];
                }
            }
        }
    }
    ans
}
