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
        m: usize,
        q: usize,
        wv: [(usize, usize); n],
        x: [usize; m],
        query: [(usize, usize); q],
    }
    println!(
        "{}",
        solve(n, m, wv, x, query)
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(
    _n: usize,
    _m: usize,
    mut wv: Vec<(usize, usize)>,
    x: Vec<usize>,
    query: Vec<(usize, usize)>,
) -> Vec<usize> {
    let mut ans = vec![];
    wv.sort_by_key(|v| -(v.1 as i64));
    for (l, r) in query {
        let mut boxes = vec![];
        for (i, &w) in x.iter().enumerate() {
            if l - 1 <= i && i <= r - 1 {
                continue;
            }
            boxes.push(w);
        }
        boxes.sort();
        let mut sum = 0;
        for &(w, v) in &wv {
            let mut index = None;
            for (i, &b) in boxes.iter().enumerate() {
                if b >= w {
                    sum += v;
                    index = Some(i);
                    break;
                }
            }
            if let Some(i) = index {
                boxes.remove(i);
            }
        }
        ans.push(sum);
    }

    ans
}
