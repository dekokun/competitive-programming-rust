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
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }
    println!(
        "{}",
        solve(n, cp, q, lr)
            .into_iter()
            .map(|a| a.0.to_string() + " " + &a.1.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(
    n: usize,
    cp: Vec<(usize, usize)>,
    _q: usize,
    lr: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut c1 = vec![0; n];
    let mut c2 = vec![0; n];
    for (i, (c, p)) in cp.into_iter().enumerate() {
        if c == 1 {
            c1[i] = p;
        } else {
            c2[i] = p;
        }
    }
    debug!(c1, c2);
    let mut c1_cumsum = vec![0];
    let mut c2_cumsum = vec![0];
    let mut c1_now = 0;
    for p in c1 {
        c1_now += p;
        c1_cumsum.push(c1_now);
    }
    let mut c2_now = 0;
    for p in c2 {
        c2_now += p;
        c2_cumsum.push(c2_now);
    }
    let mut ans = vec![];
    for (l, r) in lr {
        ans.push((
            c1_cumsum[r] - c1_cumsum[l - 1],
            c2_cumsum[r] - c2_cumsum[l - 1],
        ))
    }
    ans
}
