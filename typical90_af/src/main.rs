#![allow(non_snake_case, unused_macros, dead_code)]

use itertools::Itertools;

use proconio::input;
use proconio::marker::Usize1;

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
        a: [[i32; n]; n],
        m: usize,
        x: [(Usize1, Usize1); m],
    }
    println!("{}", solve(n, a, x));
}

fn solve(n: usize, a: Vec<Vec<i32>>, x: Vec<(usize, usize)>) -> i32 {
    let mut m = vec![vec![false; n]; n];
    for (x, y) in x {
        m[x][y] = true;
        m[y][x] = true;
    }
    let max = 1000000;
    let mut ans = max;
    'outer: for perm in (0..n).permutations(n) {
        debug!(perm);
        for v in perm.windows(2) {
            if m[v[0]][v[1]] {
                debug!("skipped");
                continue 'outer;
            }
        }
        let mut tmp = 0;
        for (i, v) in perm.into_iter().enumerate() {
            tmp += a[v][i];
        }
        ans = ans.min(tmp);
        debug!(tmp, ans);
    }
    if ans == max {
        -1
    } else {
        ans
    }
}
