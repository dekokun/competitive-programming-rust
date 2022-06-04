#![allow(non_snake_case, unused_macros, dead_code)]

use proconio::input;

// https://maguro.dev/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
    }
    println!("{}", solve(n).join("\n"));
}

fn solve(n: usize) -> Vec<String> {
    let mut ans: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        if i == 0 {
            ans.push(vec![1]);
            continue;
        }
        let mut tmp = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                tmp.push(1);
                continue;
            }
            tmp.push(ans[i - 1][j - 1] + ans[i - 1][j]);
        }
        ans.push(tmp);
    }
    ans.into_iter()
        .map(|a| {
            a.into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}
