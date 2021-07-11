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
        map: [[usize; w]; h],
    }
    println!(
        "{}",
        solve(h, w, map)
            .into_iter()
            .map(|vec| vec
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(h: usize, w: usize, map: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut row_sums = vec![];
    for i in 0..h {
        let mut sum = 0;
        for j in 0..w {
            sum += map[i][j];
        }
        row_sums.push(sum);
    }
    let mut column_sums = vec![];
    for i in 0..w {
        let mut sum = 0;
        for j in 0..h {
            sum += map[j][i];
        }
        column_sums.push(sum);
    }
    debug!(row_sums, column_sums);
    let mut ans = vec![];
    for i in 0..h {
        let mut row = vec![];
        for j in 0..w {
            row.push(row_sums[i] + column_sums[j] - map[i][j])
        }
        ans.push(row);
    }
    ans
}
