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
        let mut l = -1;
        let mut r = a.len() as i32;
        while r - l > 1 {
            let m = (r + l) / 2;
            if v < a[m as usize] {
                r = m;
            } else {
                l = m;
            }
        }
        ans.push(if r == a.len() as i32 {
            v - a[a.len() - 1]
        } else if l == -1 {
            a[0] - v
        } else {
            (a[r as usize] - v).min(v - a[l as usize])
        });
    }
    ans
}
