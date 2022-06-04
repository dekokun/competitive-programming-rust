#![allow(non_snake_case, unused_macros, dead_code)]

use std::collections::{HashMap, HashSet};

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
        k: usize,
        a: [usize; n]
    }
    println!("{}", solve(k, a));
}

fn solve(k: usize, a: Vec<usize>) -> String {
    if k == 1 {
        return "Yes".into();
    }
    let mut b = a.clone();
    b.sort();
    let b = b.into_iter().enumerate().collect::<Vec<_>>();
    let mut m = HashMap::new();
    for (i, v) in b {
        let e = m.entry(v).or_insert_with(|| (i, i));
        e.1 = i;
    }
    let mut comp = HashSet::new();
    for (i, v) in a.into_iter().enumerate() {
        let &(min, max) = m.get(&v).unwrap();
        let rem = i % k;
        let mut hit = if min <= (min / k) * k + rem && (min / k) * k + rem <= max {
            (min / k) * k + rem
        } else if min <= (min / k) * k + rem + k && (min / k) * k + rem + k <= max {
            (min / k) * k + rem + k
        } else {
            return "No".into();
        };
        if comp.contains(&hit) {
            loop {
                hit += k;
                if hit <= max && !comp.contains(&hit) {
                    break;
                }
                if hit > max {
                    return "No".into();
                }
            }
        }
        comp.insert(hit);
    }
    "Yes".into()
}
