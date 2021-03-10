#![allow(non_snake_case, unused_macros)]

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
        a: usize,
        b: usize,
        q: usize,
        s: [i64; a],
        t: [i64; b],
        x: [i64; q],
    }
    println!(
        "{}",
        solve(s, t, x)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(mut s: Vec<i64>, mut t: Vec<i64>, x: Vec<i64>) -> Vec<i64> {
    let x_max = x.iter().max().unwrap();
    // 番兵
    let s_max = s.iter().max().unwrap().max(x_max).clone();
    s.push(-2 * s_max);
    s.push(s_max * 3);
    let t_max = t.iter().max().unwrap().max(x_max).clone();
    t.push(-2 * t_max);
    t.push(t_max * 3);
    s.sort();
    t.sort();
    debug!(s, t);
    let mut ans = vec![];
    for v in x {
        debug!(v);
        let sr = s.binary_search(&v);
        let tr = t.binary_search(&v);
        let a = match (sr, tr) {
            (Ok(_), Ok(_)) => 0,
            (Ok(_), Err(t1)) => {
                let max = t[t1];
                let min = t[t1 - 1];
                (max - v).min(v - min)
            }
            (Err(s1), Ok(_)) => {
                let max = s[s1];
                let min = s[s1 - 1];
                (max - v).min(v - min)
            }
            (Err(s1), Err(t1)) => {
                let s_max = s[s1];
                let s_min = s[s1 - 1];
                let t_max = t[t1];
                let t_min = t[t1 - 1];
                // 小さい小さい
                let r1 = v - s_min.min(t_min);
                // 大きい大きい
                let r2 = s_max.max(t_max) - v;
                // 小さい大きい
                let r3 = (2 * (s_max - v) + (v - t_min)).min(2 * (v - t_min) + (s_max - v));
                // 大きい小さい
                let r4 = (2 * (t_max - v) + (v - s_min)).min(2 * (v - s_min) + (t_max - v));
                debug!(r1, r2, r3, r4);
                r1.min(r2).min(r3).min(r4)
            }
        };
        ans.push(a);
    }
    ans
}
