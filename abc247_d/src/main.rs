#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::VecDeque;

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
        q: usize,
    }
    let mut qs = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        let query = if t == 1 {
            input! {
                query: [usize; 2],
            }
            query
        } else {
            input! {
                query: [usize; 1],
            }
            query
        };
        qs.push(query);
    }
    println!(
        "{}",
        solve(qs)
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn solve(q: Vec<Vec<usize>>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut ans = vec![];
    for v in q {
        if v.len() == 2 {
            queue.push_back((v[0], v[1]))
        } else {
            let mut rem = v[0];
            let mut sum = 0;
            while let Some((x, c)) = queue.pop_front() {
                if rem < c {
                    queue.push_front((x, c - rem));
                    sum += x * rem;
                    break;
                } else if rem == c {
                    sum += x * rem;
                    break;
                } else {
                    sum += x * c;
                    rem -= c;
                }
            }
            ans.push(sum);
        };
    }
    ans
}
