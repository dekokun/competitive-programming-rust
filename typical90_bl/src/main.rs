#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::{input, marker::Usize1};

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
        a: [i64; n],
        lrv: [(Usize1, Usize1, i64); q],
    }
    println!(
        "{}",
        solve(a, lrv)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(a: Vec<i64>, lrv: Vec<(usize, usize, i64)>) -> Vec<i64> {
    let mut fubens = a.windows(2).map(|x| x[0] - x[1]).collect::<Vec<_>>();
    let mut ans = vec![];
    let mut fuben: i64 = fubens.iter().map(|v| v.abs()).sum();
    for (l, r, v) in lrv {
        if l != 0 {
            fuben -= fubens[l - 1].abs();
            let new_fuben = fubens[l - 1] - v;
            fuben += new_fuben.abs();
            fubens[l - 1] = new_fuben;
        }
        if r != a.len() - 1 {
            fuben -= fubens[r].abs();
            let new_fuben = fubens[r] + v;
            fuben += new_fuben.abs();
            fubens[r] = new_fuben;
        }
        ans.push(fuben);
    }
    ans
}
