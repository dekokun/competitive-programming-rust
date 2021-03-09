#![allow(non_snake_case, unused_macros)]

// https://maguro.dev/debug-macro/ から
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

use std::{collections::HashSet, str::FromStr};
use std::{
    collections::VecDeque,
    io::{stdin, Read},
};
fn read_option<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok()
}
fn read<T: FromStr>() -> T {
    let opt = read_option();
    opt.expect("failed to parse token")
}

fn main() {
    let n: usize = read();
    println!(
        "{}",
        solve(n, read(), read(), read(), (0..n).map(|_| read()).collect())
    );
}

fn solve(n: usize, a: usize, b: usize, c: usize, mut l: Vec<usize>) -> usize {
    let mut q = VecDeque::new();
    let mut exist = HashSet::new();
    l.sort();
    q.push_front(l.clone());
    exist.insert(l);
    while let Some(l) = q.pop_back() {
        for i in 0..l.len() {
            for j in 0..i {
                let mut new_vec = vec![l[j] + l[i]];
                for k in 0..l.len() {
                    if k == i || k == j {
                        continue;
                    }
                    new_vec.push(l[k]);
                }
                new_vec.sort();
                if exist.contains(&new_vec) {
                    continue;
                }
                exist.insert(new_vec.clone());
                q.push_front(new_vec);
            }
        }
    }
    let mut ans = std::usize::MAX;
    for vec in exist {
        if vec.len() < 3 {
            continue;
        }
        let merge_mp = (n - vec.len()) * 10;
        for i in 0..vec.len() {
            for j in i + 1..vec.len() {
                for k in j + 1..vec.len() {
                    ans = ans.min(
                        vec[i].max(c) - vec[i].min(c) + vec[j].max(b) - vec[j].min(b)
                            + vec[k].max(a)
                            - vec[k].min(a)
                            + merge_mp,
                    );
                    if vec[k] > a {
                        break;
                    }
                }
            }
        }
    }
    ans
}
