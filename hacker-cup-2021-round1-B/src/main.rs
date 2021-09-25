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
        T: usize,
    }
    for t in 1..=T {
        input! {
            n: usize,
            m: usize,
            a: usize,
            b: usize
        }
        println!("Case #{}: {}", t, solve(n, m, a, b));
    }
}

fn solve(n: usize, m: usize, a: usize, b: usize) -> String {
    if n + m - 1 > a || n + m - 1 > b {
        return "Impossible".into();
    }
    let mut ans = vec![vec![1; m]; n];
    ans[0][0] = a - (n + m - 2);
    ans[0][m - 1] = b - (n + m - 2);
    return "Possible\n".to_string()
        + &ans
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n");
}
