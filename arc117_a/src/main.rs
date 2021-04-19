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
        a: usize,
        b: usize
    }
    println!(
        "{}",
        solve(a, b)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solve(a: usize, b: usize) -> Vec<i64> {
    let mut ans = vec![];
    let (max, min) = (a.max(b), a.min(b));
    let mut max_sum = 0;
    let mut min_sum = 0;
    for i in 1..=max {
        ans.push(i as i64);
        max_sum += i;
    }
    for i in 1..=min - 1 {
        ans.push(-(i as i64));
        min_sum += i;
    }
    debug!(max_sum, min_sum);
    ans.push(((max_sum - min_sum) as i64) * -1);
    let ans = if b > a {
        ans.into_iter().map(|a| -a).collect()
    } else {
        ans
    };
    ans
}
