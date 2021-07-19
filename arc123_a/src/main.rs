#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/ $B$+$i(B
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        a1: i64,
        a2: i64,
        a3: i64,
    }
    println!("{}", solve(a1, a2, a3));
}

fn solve(a1: i64, a2: i64, a3: i64) -> i64 {
    let mut ans = 0;
    let mut d1 = a2 - a1;
    let mut d2 = a3 - a2;
    while d1 != d2 {
        // a2を増やす
        if d2 - d1 >= 2 {
            let diff = d2 - d1;
            let count = diff / 2;
            d1 += count;
            d2 -= count;
            ans += count;
        } else if d2 - d1 == 1 {
            // a3を増やす
            d2 += 1;
            ans += 1;
        } else if d2 < d1 {
            // a1 を増やす
            let diff = d1 - d2;
            d1 -= diff;
            ans += diff;
        }
    }
    ans
}
