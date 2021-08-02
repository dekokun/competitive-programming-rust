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
        h: usize,
        w: usize,
        a: [[isize; w]; h],
        b: [[isize; w]; h],
    }
    println!(
        "{}",
        match solve(a, b) {
            None => "No".into(),
            Some(a) => format!("Yes\n{}", a),
        }
    );
}

fn solve(mut a: Vec<Vec<isize>>, b: Vec<Vec<isize>>) -> Option<isize> {
    let mut ans = 0;
    for i in 0..a.len() - 1 {
        for j in 0..a[0].len() - 1 {
            if a[i][j] == b[i][j] {
                continue;
            }
            if a[i][j] > b[i][j] {
                let diff = a[i][j] - b[i][j];
                ans += diff;
                a[i][j] -= diff;
                a[i + 1][j] -= diff;
                a[i][j + 1] -= diff;
                a[i + 1][j + 1] -= diff;
            }
            if a[i][j] < b[i][j] {
                let diff = b[i][j] - a[i][j];
                ans += diff;
                a[i][j] += diff;
                a[i + 1][j] += diff;
                a[i][j + 1] += diff;
                a[i + 1][j + 1] += diff;
            }
        }
    }
    for i in 0..a.len() {
        for j in 0..a[0].len() {
            if a[i][j] != b[i][j] {
                return None;
            }
        }
    }
    Some(ans)
}
