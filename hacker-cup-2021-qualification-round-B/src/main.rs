#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

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
            m: [Chars; n]
        }
        println!("Case #{}: {}", t, solve(n, m));
    }
}

fn solve(n: usize, m: Vec<Vec<char>>) -> String {
    let inf = 2 * n + 1;
    let mut count = inf;
    let mut combi = HashSet::new();
    'outer: for i in 0..n {
        let mut tmp_count = 0;
        let mut tmp_combi = vec![];
        for j in 0..n {
            let now = m[i][j];
            match now {
                'O' => continue 'outer,
                'X' => continue,
                '.' => {
                    tmp_count += 1;
                    tmp_combi.push((i, j));
                }
                _ => unreachable!(),
            }
        }
        if tmp_count < count {
            count = tmp_count;
            combi.clear();
            combi.insert(tmp_combi);
        } else if tmp_count == count {
            combi.insert(tmp_combi);
        }
    }
    'outer2: for i in 0..n {
        let mut tmp_count = 0;
        let mut tmp_combi = vec![];
        for j in 0..n {
            let now = m[j][i];
            match now {
                'O' => continue 'outer2,
                'X' => continue,
                '.' => {
                    tmp_count += 1;
                    tmp_combi.push((j, i));
                }
                _ => unreachable!(),
            }
        }
        if tmp_count < count {
            count = tmp_count;
            combi.clear();
            combi.insert(tmp_combi);
        } else if tmp_count == count {
            combi.insert(tmp_combi);
        }
    }
    if count == inf {
        "Impossible".into()
    } else {
        format!("{} {}", count, combi.len())
    }
}
